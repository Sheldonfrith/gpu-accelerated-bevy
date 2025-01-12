use crate::{
    state::ModuleTransformState,
    transformer::{allowed_types::AllowedRustTypes, to_wgsl_syntax::convert_to_wgsl},
};
use proc_macro::Span;
use proc_macro_error::abort;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::quote;
use shared::wgsl_components::{
    WgslConstAssignment, WgslFunction, WgslShaderModuleComponent, WgslShaderModuleUserPortion,
    WgslType,
};
use syn::{
    Ident, Item, ItemConst, ItemFn, ItemMod, PatIdent, parse_quote,
    spanned::Spanned,
    visit::{self, Visit},
};

pub fn find_main_function(mut state: &mut ModuleTransformState) {
    let module = state.rust_module.clone();
    let mut extractor = MainFunctionsExtractor::new(&mut state);
    extractor.visit_item_mod(&module);
    let main_func = if let Some(mf) = &state.result.main_function {
        mf
    } else {
        abort!(state.rust_module.ident.span(), "No main function found");
    };
    let r_code = main_func.code.rust_code.clone();
    validate_main_function(r_code);
    state.rust_module = module;
}

struct MainFunctionsExtractor<'a> {
    count: usize,
    state: &'a mut ModuleTransformState,
}

impl<'ast> Visit<'ast> for MainFunctionsExtractor<'ast> {
    fn visit_item_fn(&mut self, c: &'ast syn::ItemFn) {
        syn::visit::visit_item_fn(self, c);
        let name = c.sig.ident.to_string();
        if name != "main" {
            return;
        }
        self.count += 1;
        if self.count > 1 {
            abort!(c.sig.ident.span(), "Only one main function is allowed");
        }
        self.state.result.main_function = Some(parse_main_fn(c, self.state));
    }
}

impl<'ast> MainFunctionsExtractor<'ast> {
    pub fn new(state: &'ast mut ModuleTransformState) -> Self {
        MainFunctionsExtractor { count: 0, state }
    }
}

fn parse_main_fn(func: &ItemFn, state: &ModuleTransformState) -> WgslFunction {
    let mut func_clone = func.clone();
    // alter the main function argument
    WgslFunction {
        code: WgslShaderModuleComponent {
            rust_code: func_clone.to_token_stream().to_string(),
            wgsl_code: alter_global_id_argument(convert_to_wgsl(
                func_clone.to_token_stream(),
                state,
            )),
        },
        name: func_clone.sig.ident.to_string(),
    }
}
/// we have to alter the main function argument to match the wgsl spec by string replace instead of ast manipulation because the new argument is not a valid rust syntax
fn alter_global_id_argument(func_string: String) -> String {
    let match_patterns = [
        "global_id: WgslGlobalId",
        "global_id : WgslGlobalId",
        "global_id:WgslGlobalId",
        "global_id:  WgslGlobalId",
    ];
    let replace_pattern = "@builtin(global_invocation_id) global_id: vec3<u32>";
    let mut new_func = func_string.clone();
    let mut found = false;
    for pattern in match_patterns.iter() {
        if new_func.find(pattern).is_some() {
            found = true;
            new_func = new_func.replace(pattern, replace_pattern);
        }
    }
    if !found {
        let error_message = format!(
            "Failed to find main function argument, we are looking for a string that exactly matches 'global_id: WgslGlobalId', found {}",
            new_func
        );
        abort!(Span::call_site(), error_message);
    }
    new_func
}

fn validate_main_function(function_string: String) {
    let function = if let Ok(f) = syn::parse_str::<ItemFn>(&function_string) {
        f
    } else {
        let message = format!("Failed to parse main function: {}", function_string);
        abort!(Span::call_site(), message);
    };
    // Check that main has exactly one parameter
    if function.sig.inputs.len() != 1 {
        abort!(
            function.sig.span(),
            "Main function must have exactly one parameter of type WgslGlobalId"
        );
    }
    // Validate the parameter type is WgslGlobalId called "global_id"
    if let syn::FnArg::Typed(pat_type) = &function.sig.inputs[0] {
        match &*pat_type.pat {
            syn::Pat::Ident(_) => {}
            _ => abort!(
                pat_type.pat.span(),
                "Main function parameter must be called 'global_id'"
            ),
        }
        if let syn::Type::Path(type_path) = &*pat_type.ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident != "WgslGlobalId" {
                    abort!(
                        pat_type.ty.span(),
                        "Main function parameter must be of type WgslGlobalId"
                    );
                }
            }
        }
    }
    // Check return type (should be void/unit)
    if let syn::ReturnType::Type(_, _) = &function.sig.output {
        abort!(
            function.sig.span(),
            "Main function cannot have a return type"
        );
    }
}

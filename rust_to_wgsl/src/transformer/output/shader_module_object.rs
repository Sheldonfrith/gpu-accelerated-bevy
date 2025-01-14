use crate::{
    state::ModuleTransformState, transformer::output::per_component_expansion::ToStructInitializer,
};
use proc_macro_error::abort;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use shared::wgsl_components::WgslShaderModuleUserPortion;

pub fn generate_shader_module_object(state: &ModuleTransformState) -> TokenStream {
    let obj: WgslShaderModuleUserPortion = state.result.clone();

    let static_consts: TokenStream = obj
        .static_consts
        .into_iter()
        .map(|const_assignment| {
            let ts = ToStructInitializer::wgsl_const_assignment(const_assignment);
            quote!(#ts,)
        })
        .collect();

    let helper_types: TokenStream = obj
        .helper_types
        .into_iter()
        .map(|type_def| {
            let ts = ToStructInitializer::wgsl_type(type_def);
            quote!(#ts,)
        })
        .collect();

    let uniforms2: TokenStream = obj
        .uniforms
        .into_iter()
        .map(|uniform| {
            let ts = ToStructInitializer::wgsl_type(uniform);
            quote!(#ts,)
        })
        .collect();

    let input_arrays: TokenStream = obj
        .input_arrays
        .into_iter()
        .map(|array| {
            let ts = ToStructInitializer::wgsl_input_array(array);
            quote!(#ts,)
        })
        .collect();

    let output_arrays: TokenStream = obj
        .output_arrays
        .into_iter()
        .map(|output_array| {
            let ts = ToStructInitializer::wgsl_output_array(output_array);
            quote!(#ts,)
        })
        .collect();

    let helper_functions: TokenStream = obj
        .helper_functions
        .into_iter()
        .map(|func| {
            let ts = ToStructInitializer::wgsl_function(func);
            quote!(#ts,)
        })
        .collect();

    let main_function: TokenStream = obj.main_function.map_or(quote!(None), |func| {
        let ts = ToStructInitializer::wgsl_function(func);
        quote!(Some(#ts))
    });

    quote!(
        pub fn parsed() -> WgslShaderModuleUserPortion {
            WgslShaderModuleUserPortion {
                static_consts: [
                    #static_consts
                    ]
                .into(),
                helper_types: [
                    #helper_types
                    ]
                .into(),
                uniforms: Vec::from([
                   #uniforms2
                    ]),
                input_arrays: [
                    #input_arrays
                    ]
                .into(),
                output_arrays: [
                    #output_arrays
                    ]
                .into(),
                helper_functions: [
                    #helper_functions
                    ]
                .into(),
                // main_function: None,
                main_function: #main_function,
            }
        }
    )
}

#[cfg(test)]

mod test {
    use proc_macro_error::abort;
    use proc_macro2::{Span, TokenStream};

    #[test]
    pub fn test_parse_str() {
        let uniforms_str2 = "";
        let uniforms2: TokenStream = if let Ok(ts) = uniforms_str2.parse() {
            ts
        } else {
            abort!(
                Span::call_site(),
                "Failed to parse uniforms into TokenStream"
            );
        };
    }
}

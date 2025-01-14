use proc_macro2::{Span, TokenStream};
use quote::quote;
use shared::{
    custom_type_name::CustomTypeName,
    wgsl_components::{
        WgslArrayLength, WgslConstAssignment, WgslDerivedType, WgslFunction, WgslInputArray,
        WgslOutputArray, WgslShaderModuleComponent, WgslType,
    },
};
use syn::Ident;
pub struct ToStructInitializer {}

impl ToStructInitializer {
    pub fn wgsl_shader_module_component(c: WgslShaderModuleComponent) -> TokenStream {
        let r = c.rust_code;
        let w = c.wgsl_code;
        quote!(
            WgslShaderModuleComponent {
                rust_code: (#r).to_string(),
                wgsl_code: (#w).to_string(),
            }
        )
    }

    pub fn wgsl_type(c: WgslType) -> TokenStream {
        let n = ToStructInitializer::custom_type_name(c.name);
        let c = ToStructInitializer::wgsl_shader_module_component(c.code);
        quote!(
            WgslType {
                name: #n,
                code: #c,
            }
        )
        .into()
    }

    pub fn custom_type_name(c: CustomTypeName) -> TokenStream {
        let n = c.name;
        let u = c.upper;
        let l = c.lower;
        quote!(
            CustomTypeName {
                name: (#n).to_string(),
                upper: (#u).to_string(),
                lower: (#l).to_string(),
            }
        )
    }

    pub fn wgsl_derived_type(c: WgslDerivedType) -> TokenStream {
        let n = c.name;
        let c = ToStructInitializer::wgsl_shader_module_component(c.code);
        quote!(
            WgslDerivedType {
                name: (#n).to_string(),
                code: #c,
            }
        )
    }

    pub fn wgsl_function(c: WgslFunction) -> TokenStream {
        let n = c.name;
        let c = ToStructInitializer::wgsl_shader_module_component(c.code);
        quote!(
            WgslFunction {
                name: (#n).to_string(),
                code: #c
            }
        )
    }

    pub fn wgsl_const_assignment(c: WgslConstAssignment) -> TokenStream {
        let c = ToStructInitializer::wgsl_shader_module_component(c.code);
        quote!(
            WgslConstAssignment {
                code: #c,
            }
        )
    }
    pub fn wgsl_array_length(c: WgslArrayLength) -> TokenStream {
        let n = c.name;
        let c = ToStructInitializer::wgsl_shader_module_component(c.code);
        quote!(
            WgslArrayLength {
                name: (#n).to_string(),
                code: #c,
            }
        )
    }

    pub fn wgsl_input_array(c: WgslInputArray) -> TokenStream {
        let i = ToStructInitializer::wgsl_type(c.item_type);
        let a = ToStructInitializer::wgsl_derived_type(c.array_type);
        quote!(
            WgslInputArray {
                item_type: #i,
                array_type: #a,
            }
        )
    }

    pub fn wgsl_output_array(c: WgslOutputArray) -> TokenStream {
        let i = ToStructInitializer::wgsl_type(c.item_type);
        let a = ToStructInitializer::wgsl_derived_type(c.array_type);
        let ac: TokenStream = c
            .atomic_counter_name
            .as_ref()
            .map_or("None".to_string(), |counter| {
                format!("Some(\"{}\".to_string())", counter)
            })
            .to_string()
            .parse()
            .unwrap();
        quote!(
            WgslOutputArray {
                item_type: #i,
                array_type: #a,
                atomic_counter_name: #ac

            }
        )
    }
}

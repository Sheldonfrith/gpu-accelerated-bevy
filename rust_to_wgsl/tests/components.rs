#![feature(f16)]
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use rust_to_wgsl::wgsl_shader_module;
use shared::{
    misc_types::TypesSpec, wgsl_components::WgslShaderModuleComponent,
    wgsl_in_rust_helpers::Vec3Bool,
};
use syn::{ItemMod, parse_quote};

#[test]
fn test_simple_struct() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::WgslIterationPosition;

        struct TStruct {
            x: f32,
            y: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            return;
        }
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>) { return; }"
    );
}

#[test]
fn test_struct_creation() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::WgslIterationPosition;

        struct TStruct {
            x: f32,
            y: f32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let obj = TStruct { x: 1.0, y: 2.0 };
            return;
        }
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{ let obj = TStruct(1.0, 2.0); return; }"
    );
}

#[test]
fn test_struct_creation_with_nested_transforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::*;

        struct TStruct {
            x: f32,
            y: Vec3F32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let obj = TStruct {
                x: 1.0,
                y: Vec3F32::new(2.0, 3.0, 4.0),
            };
            return;
        }
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);

    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.main_function.unwrap().code.wgsl_code,
        "fn main(@builtin(global_invocation_id) iter_pos: vec3<u32>)\n{ let obj = TStruct(1.0,vec3<f32>(2.0, 3.0, 4.0)); return; }"
    );
}
#[test]
fn test_type_alias() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::*;
        type MyType = i32;
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.helper_types.first().unwrap().code.wgsl_code,
        "alias MyType  = i32;"
    );
}
#[test]
fn test_consts() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::{WgslIterationPosition, *};
        const MY_CONST: i32 = 3;
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 1);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.static_consts.first().unwrap().code.wgsl_code,
        "const MY_CONST : i32 = 3;"
    );
}
#[test]
fn test_uniforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_config;
        use shared::wgsl_in_rust_helpers::*;
        #[wgsl_config]
        struct Uniforms {
            time: f32,
            resolution: Vec2F32,
        }
        fn main(iter_pos: WgslIterationPosition) {
            let time = WgslConfigInput::get::<Uniforms>().time;
        }
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 1);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.uniforms.first().unwrap().code.wgsl_code,
        "struct Uniforms { time : f32, resolution : vec2 < f32 > , }"
    );
}

#[test]
fn test_output_arrays() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_output_array;
        use shared::wgsl_in_rust_helpers::*;
        #[wgsl_output_array]
        struct CollisionResult {
            entity1: u32,
            entity2: u32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 1);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.output_arrays.first().unwrap().item_type.code.wgsl_code,
        "struct CollisionResult { entity1 : u32, entity2 : u32, }"
    );
    assert_eq!(
        t2.output_arrays.first().unwrap().array_type.code.wgsl_code,
        "alias collisionresult_output_array  = array < CollisionResult,
COLLISIONRESULT_OUTPUT_ARRAY_LENGTH > ;"
    );
    assert!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .is_none()
    );
}

#[test]
fn test_helper_functions() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::*;
        fn calculate_distance_squared(p1: [f32; 2], p2: [f32; 2]) -> f32 {
            let dx = p1[0] - p2[0];
            let dy = p1[1] - p2[1];
            return dx * dx + dy * dy;
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 1);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.helper_functions.first().unwrap().code.wgsl_code,
        "fn calculate_distance_squared(p1 : array < f32, 2 > , p2 : array < f32, 2 >)\n-> f32\n{\n    let dx = p1 [0] - p2 [0]; let dy = p1 [1] - p2 [1]; return dx * dx + dy *\n    dy;\n}"
    );
}

#[test]

fn t() {}

#[test]
// expect a panic
#[should_panic("not implemented")]
fn can_extract_types() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_config;
        use shared::wgsl_in_rust_helpers::*;
        /// some doc comment, should be removed
        #[wgsl_config]
        struct MyConfig {
            value: PodF16,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    fn fun<T: TypesSpec>() -> T::InputConfigTypes {
        unimplemented!();
    }
    let t = fun::<test_module::Types>();
}

#[test]
fn test_simple_type_transforms() {
    #[wgsl_shader_module]
    pub mod test_module {
        use shared::wgsl_in_rust_helpers::{WgslIterationPosition, *};
        struct TStruct {
            x: f32,
            y: Vec3F32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.helper_types.len() == 1);
    assert_eq!(
        t2.helper_types.first().unwrap().code.wgsl_code,
        "struct TStruct { x : f32, y : vec3 < f32 > , }"
    );
}

#[test]
fn test_doc_comments() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_config;
        use shared::wgsl_in_rust_helpers::*;
        /// some doc comment, should be removed
        #[wgsl_config]
        struct MyConfig {
            f16_val: PodF16,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 1);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    let t = WgslShaderModuleComponent {
        rust_code: ("#[wgsl_config] struct MyConfig { value : PodBool, }").to_string(),
        wgsl_code: ("struct MyConfig { value : bool, }").to_string(),
    };
}
#[test]
fn test_input_arrays() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_input_array;
        use shared::wgsl_in_rust_helpers::*;
        #[wgsl_input_array]
        type Position = [f32; 2];
        fn main(iter_pos: WgslIterationPosition) {}
    }

    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 0);
    assert!(t2.input_arrays.len() == 1);
    assert!(t2.uniforms.len() == 0);
    // type Position = array<f32, 2>;
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.input_arrays.first().unwrap().array_type.code.wgsl_code,
        "alias position_input_array  = array < Position, POSITION_INPUT_ARRAY_LENGTH > ;"
    );
    assert_eq!(
        t2.input_arrays.first().unwrap().item_type.code.wgsl_code,
        "alias Position  = array < f32, 2 > ;"
    )
}

#[test]
fn test_output_vec() {
    #[wgsl_shader_module]
    pub mod test_module {
        use rust_to_wgsl::wgsl_output_vec;
        use shared::wgsl_in_rust_helpers::*;
        #[wgsl_output_vec]
        struct CollisionResult {
            entity1: u32,
            entity2: u32,
        }
        fn main(iter_pos: WgslIterationPosition) {}
    }
    let t2 = test_module::parsed();
    assert!(t2.output_arrays.len() == 1);
    assert!(t2.input_arrays.len() == 0);
    assert!(t2.uniforms.len() == 0);
    assert!(t2.helper_functions.len() == 0);
    assert!(t2.main_function.is_some());
    assert!(t2.static_consts.len() == 0);
    assert!(t2.helper_types.len() == 0);
    assert_eq!(
        t2.output_arrays.first().unwrap().item_type.code.wgsl_code,
        "struct CollisionResult { entity1 : u32, entity2 : u32, }"
    );
    assert_eq!(
        t2.output_arrays.first().unwrap().array_type.code.wgsl_code,
        "alias collisionresult_output_array  = array < CollisionResult,
COLLISIONRESULT_OUTPUT_ARRAY_LENGTH > ;"
    );
    assert!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .is_some()
    );
    assert_eq!(
        t2.output_arrays
            .first()
            .unwrap()
            .atomic_counter_name
            .as_ref()
            .unwrap(),
        &"collisionresult_counter".to_string()
    )
}

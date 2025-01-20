use std::{borrow::Cow, default};

use bevy::{
    ecs::batching::BatchingStrategy,
    log,
    prelude::{Changed, EventReader, Query, Res},
    render::{
        render_resource::{ComputePipelineDescriptor, PipelineCache},
        renderer::RenderDevice,
    },
};
use shared::wgsl_components::{
    WORKGROUP_SIZE_X_VAR_NAME, WORKGROUP_SIZE_Y_VAR_NAME, WORKGROUP_SIZE_Z_VAR_NAME,
};
use wgpu::PipelineCompilationOptions;

use crate::task::{
    events::{GpuComputeTaskChangeEvent, IterSpaceOrOutputSizesChangedEvent},
    task_components::task_name::TaskName,
    task_specification::task_specification::ComputeTaskSpecification,
    wgsl_code::WgslCode,
};

use super::{
    cache::{PipelineKey, PipelineLruCache},
    pipeline_layout::PipelineLayoutComponent,
    shader_module::{self, shader_module_from_wgsl_string},
};

pub fn update_pipelines_on_pipeline_const_change(
    mut tasks: Query<(
        &TaskName,
        &ComputeTaskSpecification,
        &PipelineLayoutComponent,
        &mut PipelineLruCache,
    )>,
    mut wgsl_code_changed_event_reader: EventReader<IterSpaceOrOutputSizesChangedEvent>,
    render_device: Res<RenderDevice>,
) {
    log::info!("update_pipelines_on_pipeline_const_change");
    for (ev, _) in wgsl_code_changed_event_reader
        .par_read()
        .batching_strategy(BatchingStrategy::default())
    {
        let task = tasks.get_mut(ev.entity().clone());
        if let Ok((task_name, task_spec, pipeline_layout, mut pipeline_cache)) = task {
            update_single_pipeline(
                task_spec,
                task_name,
                &render_device,
                &pipeline_layout,
                &mut pipeline_cache,
            );
        }
    }
}

fn update_single_pipeline(
    spec: &ComputeTaskSpecification,
    task_name: &TaskName,
    render_device: &RenderDevice,
    pipeline_layout: &PipelineLayoutComponent,
    // pipeline_cache: &mut PipelineLruCache,
    pipeline_cache: Res<PipelineCache>,
) {
    log::info!("Updating pipeline for task {}", task_name.get());
    let key = PipelineKey {
        pipeline_consts_version: spec.iter_space_and_out_lengths_version(),
    };
    if pipeline_cache.cache.contains_key(&key) {
        return;
    } else {
        log::info!("Creating new pipeline for task {}", task_name.get());
        log::info!(" layout {:?}", pipeline_layout.0);
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from(task_name.get())),
            layout: vec![pipeline_layout.0.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });
        let compute_pipeline = render_device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some(&task_name.get()),
            layout: Some(&pipeline_layout.0),
            module: spec.wgsl_code().shader_module(),
            entry_point: Some(spec.wgsl_code().entry_point_function_name()),
            // this is where we specify new values for pipeline constants...
            compilation_options: PipelineCompilationOptions {
                constants: &&spec.get_pipeline_consts(),
                zero_initialize_workgroup_memory: Default::default(),
            },
            cache: None,
        });
        pipeline_cache.cache.insert(key, compute_pipeline);
    }
}

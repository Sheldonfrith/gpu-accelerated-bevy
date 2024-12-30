use bevy::{
    prelude::{Res, ResMut},
    render::renderer::RenderDevice,
};
use wgpu::{BufferDescriptor, BufferUsages, util::BufferInitDescriptor};

use crate::code::gpu_power_user::{
    iteration_space_dependent_resources::resources::MaxNumGpuOutputItemsPerOutputType,
    wgsl_processable_types::{WgslCollisionResult, WgslCounter},
};

use super::resources::{SingleBatchBuffers, WgslInputData};

pub fn create_buffers(
    data: Res<WgslInputData>,
    render_device: Res<RenderDevice>,
    max_num_outputs: Res<MaxNumGpuOutputItemsPerOutputType>,
    mut buffers: ResMut<SingleBatchBuffers>,
) {
    // input buffers
    for input_data_types in data.input_data_types.iter() {
        //    needs a label, contents
    }
    for output_data_types in data.output_data_types.iter() {
        // needs a label, max number of result entries (to calculate size)
    }
    let positions_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Positions Buffer"),
        contents: bytemuck::cast_slice(&data.positions.positions),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let radii_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Radii Buffer"),
        contents: bytemuck::cast_slice(&data.radii.radii),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
    });

    let results_size = std::mem::size_of::<WgslCollisionResult>() * max_num_results_to_receive.0;

    let results_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("Collision Results Buffer"),
        size: results_size as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let results_staging_buffer = render_device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Results Staging Buffer"),
        size: results_size as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let counter = WgslCounter { count: 0 };
    let counter_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("Counter Buffer"),
        contents: bytemuck::cast_slice(&[counter]),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
    });

    *buffers = SingleBatchBuffers {
        positions_buffer: Some(positions_buffer),
        radii_buffer: Some(radii_buffer),
        results_buffer: Some(results_buffer),
        results_staging_buffer: Some(results_staging_buffer),
        counter_buffer: Some(counter_buffer),
    };
}

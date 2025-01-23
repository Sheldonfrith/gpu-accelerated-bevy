use std::cmp::min;

use bevy::{
    log,
    render::renderer::{RenderDevice, RenderQueue},
};
use bevy_gpu_compute_core::TypeErasedArrayOutputData;

use crate::task::task_components::task::BevyGpuComputeTask;

use super::helpers::get_gpu_output_as_bytes_vec::get_gpu_output_as_bytes_vec;
use std::collections::HashMap;
/**
 * We put this all into a single system because we cannot pass the buffer slice around easily.
 * */
pub fn read_gpu_outputs(
    output_counts: Vec<Option<usize>>,
    task: &mut BevyGpuComputeTask,
    render_device: &RenderDevice,
    render_queue: &RenderQueue,
) {
    let mut bytes_per_wgsl_output_type_name: HashMap<String, Vec<u8>> = HashMap::new();

    task.spec
        .output_vectors_metadata_spec()
        .get_all_metadata()
        .iter()
        .enumerate()
        .for_each(|(i, metadata)| {
            if let Some(m) = metadata {
                let out_buffer = task.buffers.output.get(i).unwrap();
                let staging_buffer = task.buffers.output_staging.get(i).unwrap();
                let total_byte_size = min(
                    if let Some(Some(c)) = output_counts.get(i) {
                        let size = c * m.get_bytes();
                        log::info!("using output count to size buffer, size: {}", size);
                        size
                    } else {
                        usize::MAX
                    },
                    task.spec.output_array_lengths().get_by_name(m.name()) * m.get_bytes(),
                );
                log::info!("total_byte_size: {}", total_byte_size);
                if total_byte_size < 1 {
                    bytes_per_wgsl_output_type_name.insert(m.name().name().to_string(), Vec::new());
                } else {
                    let raw_bytes = get_gpu_output_as_bytes_vec(
                        &render_device,
                        &render_queue,
                        &out_buffer,
                        staging_buffer,
                        total_byte_size as u64,
                    );
                    // log::info!("raw_bytes: {:?}", raw_bytes);
                    if let Some(raw_bytes) = raw_bytes {
                        bytes_per_wgsl_output_type_name
                            .insert(m.name().name().to_string(), raw_bytes);
                    } else {
                        panic!("Failed to read output from GPU");
                    }
                }
            }
        });
    task.output_data = Some(TypeErasedArrayOutputData::new(
        bytes_per_wgsl_output_type_name,
    ));
}

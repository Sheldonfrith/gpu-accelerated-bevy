use bevy::prelude::Component;

#[derive(Component)]
pub struct PipelineLayout(pub wgpu::PipelineLayout);

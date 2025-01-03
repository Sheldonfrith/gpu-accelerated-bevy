use bevy::prelude::{Entity, Event};

use super::{
    inputs::input_vector_metadata_spec::InputVectorsMetadataSpec,
    outputs::definitions::output_vector_metadata_spec::OutputVectorsMetadataSpec,
};

#[derive(Event)]
pub struct GpuAcceleratedTaskCreatedEvent {
    pub entity: Entity,
    pub task_name: String,
}

pub trait GpuComputeTaskChangeEvent {
    fn new(entity: Entity) -> Self;
    fn entity(&self) -> Entity;
}
#[derive(Event)]
pub struct InputDataChangeEvent {
    entity: Entity,
}
impl GpuComputeTaskChangeEvent for InputDataChangeEvent {
    fn new(entity: Entity) -> Self {
        InputDataChangeEvent { entity }
    }
    fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Event)]
pub struct IterationSpaceOrMaxOutVecLengthChangedEvent {
    entity: Entity,
}
impl GpuComputeTaskChangeEvent for IterationSpaceOrMaxOutVecLengthChangedEvent {
    fn new(entity: Entity) -> Self {
        IterationSpaceOrMaxOutVecLengthChangedEvent { entity }
    }
    fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Event)]
pub struct WgslCodeChangedEvent {
    entity: Entity,
}
impl GpuComputeTaskChangeEvent for WgslCodeChangedEvent {
    fn new(entity: Entity) -> Self {
        WgslCodeChangedEvent { entity }
    }
    fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Event)]
pub struct GpuComputeTaskSuccessEvent {
    pub id: u128,
}

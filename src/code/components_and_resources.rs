use bevy::{
    math::bounding::BoundingCircle,
    prelude::{Component, Entity, Resource},
};
use sysinfo::System;

#[derive(Debug, Component)]
pub struct BoundingCircleComponent(pub BoundingCircle);

#[derive(Resource)]
pub struct SysInfo {
    pub total_mem: u64,
}

impl Default for SysInfo {
    fn default() -> Self {
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();
        SysInfo {
            total_mem: sys.total_memory(),
        }
    }
}

use std::collections::HashMap;

use bevy::prelude::Component;

#[derive(Default, Component)]
pub struct OutputCountsFromGpu(pub HashMap<String, Option<usize>>);
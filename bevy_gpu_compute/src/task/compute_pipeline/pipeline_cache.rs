use bevy::{prelude::Component, render::render_resource::ComputePipeline};

use super::lru_cache::LruCache;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct PipelineKey {
    pub pipeline_consts_version: u64,
}

#[derive(Component)]
pub struct PipelineLruCache {
    pub cache: LruCache<PipelineKey, ComputePipeline>,
}
impl Default for PipelineLruCache {
    fn default() -> Self {
        Self {
            cache: LruCache::new(10),
        }
    }
}

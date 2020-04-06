use crate::{
    pipeline::PipelineDescriptor,
    renderer::{RenderPass, Renderer},
};
use bevy_asset::Handle;
use legion::prelude::{Resources, World};

// A set of draw calls. ex: get + draw meshes, get + draw instanced meshes, draw ui meshes, etc
pub trait DrawTarget {
    fn draw(
        &self,
        world: &World,
        resources: &Resources,
        render_pass: &mut dyn RenderPass,
        pipeline_handle: Handle<PipelineDescriptor>,
    );
    fn setup(
        &mut self,
        _world: &mut World,
        _resources: &Resources,
        _renderer: &mut dyn Renderer,
        _pipeline_handle: Handle<PipelineDescriptor>,
        _pipeline_descriptor: &PipelineDescriptor,
    ) {
    }
    fn get_name(&self) -> String;
}
use super::{
    render::SPRITE_PIPELINE_HANDLE, sprite::Sprite, ColorMaterial, TextureAtlas,
    TextureAtlasSprite, QUAD_HANDLE, SPRITE_SHEET_PIPELINE_HANDLE,
};
use crate::app::ComponentSet;
use crate::asset::Handle;
use crate::render::{
    draw::Draw,
    mesh::Mesh,
    pipeline::{DynamicBinding, PipelineSpecialization, RenderPipeline, RenderPipelines},
};
use crate::transform::prelude::{Rotation, Scale, Transform, Translation};

#[derive(ComponentSet)]
pub struct SpriteComponents {
    pub sprite: Sprite,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub material: Handle<ColorMaterial>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for SpriteComponents {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                SPRITE_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // Sprite
                        DynamicBinding {
                            bind_group: 2,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            sprite: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}

#[derive(ComponentSet)]
pub struct SpriteSheetComponents {
    pub sprite: TextureAtlasSprite,
    pub texture_atlas: Handle<TextureAtlas>,
    pub draw: Draw,
    pub render_pipelines: RenderPipelines,
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub transform: Transform,
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Default for SpriteSheetComponents {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::specialized(
                SPRITE_SHEET_PIPELINE_HANDLE,
                PipelineSpecialization {
                    dynamic_bindings: vec![
                        // Transform
                        DynamicBinding {
                            bind_group: 2,
                            binding: 0,
                        },
                        // TextureAtlasSprite
                        DynamicBinding {
                            bind_group: 2,
                            binding: 1,
                        },
                    ],
                    ..Default::default()
                },
            )]),
            draw: Draw {
                is_transparent: true,
                ..Default::default()
            },
            mesh: QUAD_HANDLE,
            sprite: Default::default(),
            texture_atlas: Default::default(),
            transform: Default::default(),
            translation: Default::default(),
            rotation: Default::default(),
            scale: Default::default(),
        }
    }
}
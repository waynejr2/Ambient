use std::sync::Arc;

use ambient_app::AppBuilder;
use ambient_core::{asset_cache, camera::active_camera, hierarchy::set_component_recursive, main_scene, mesh, transform::*};
use ambient_ecs::{EntityData, World};
use ambient_gpu::{
    gpu::GpuKey,
    shader_module::{BindGroupDesc, ShaderModule},
};
use ambient_meshes::QuadMeshKey;
use ambient_model_import::model_crate::ModelCrate;
use ambient_renderer::{
    gpu_primitives, material,
    materials::flat_material::{get_flat_shader, FlatMaterial},
    primitives, renderer_shader, Material, MaterialShader, SharedMaterial, StandardShaderKey, MATERIAL_BIND_GROUP,
};
use ambient_std::{
    asset_cache::{AssetCache, SyncAssetKey, SyncAssetKeyExt},
    asset_url::AbsAssetUrl,
    cb, friendly_id,
    math::SphericalCoords,
};
use glam::*;
use wgpu::BindGroup;

#[derive(Clone, Debug)]
pub struct CustomMaterialShaderKey;
impl SyncAssetKey<Arc<MaterialShader>> for CustomMaterialShaderKey {
    fn load(&self, _assets: AssetCache) -> Arc<MaterialShader> {
        Arc::new(MaterialShader {
            id: "custom_material_shader".to_string(),
            shader: ShaderModule::new(
                "CustomMaterial",
                include_str!("material.wgsl"),
                vec![BindGroupDesc { entries: vec![], label: MATERIAL_BIND_GROUP.into() }.into()],
            ),
        })
    }
}

pub struct CustomMaterial {
    id: String,
    bind_group: wgpu::BindGroup,
}
impl CustomMaterial {
    pub fn new(assets: AssetCache) -> Self {
        let gpu = GpuKey.get(&assets);
        let layout = CustomMaterialShaderKey.get(&assets).shader.first_layout(&assets);
        Self {
            id: friendly_id(),
            bind_group: gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[],
                label: Some("CustomMaterial.bind_group"),
            }),
        }
    }
}
impl std::fmt::Debug for CustomMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomMaterial").field("id", &self.id).finish()
    }
}
impl Material for CustomMaterial {
    fn bind(&self) -> &BindGroup {
        &self.bind_group
    }
    fn id(&self) -> &str {
        &self.id
    }
}

async fn init(world: &mut World) {
    let assets = world.resource(asset_cache()).clone();

    let model = ModelCrate::local_import(&assets, &AbsAssetUrl::parse("assets/Soldier.glb").unwrap(), true, false).await.unwrap();
    let entity = model.spawn(world, &Default::default());
    set_component_recursive(
        world,
        entity,
        renderer_shader(),
        cb(|assets, config| {
            StandardShaderKey { material_shader: CustomMaterialShaderKey.get(assets), lit: true, shadow_cascades: config.shadow_cascades }
                .get(assets)
        }),
    );
    set_component_recursive(world, entity, material(), SharedMaterial::new(CustomMaterial::new(assets.clone())));
    // world.add_component(entity, rotation(), glam::Quat::from_rotation_x(std::f32::consts::PI / 2.)).unwrap();
    // world.set(entity, animation_controller(), AnimationController::looping("Walk")).unwrap();

    let grey = FlatMaterial::new(assets.clone(), vec4(0.5, 0.5, 0.5, 1.), Some(false));

    EntityData::new()
        .set(mesh(), QuadMeshKey.get(&assets))
        .set(renderer_shader(), cb(get_flat_shader))
        .set(material(), SharedMaterial::new(grey))
        .set(primitives(), vec![])
        .set_default(gpu_primitives())
        .set_default(local_to_world())
        .set_default(mesh_to_world())
        .set(scale(), vec3(20., 20., 1.))
        .set(main_scene(), ())
        .spawn(world);

    ambient_cameras::spherical::new(vec3(0., 0., 0.), SphericalCoords::new(std::f32::consts::PI / 4., std::f32::consts::PI / 4., 5.))
        .set(active_camera(), 0.)
        .set(main_scene(), ())
        .spawn(world);
}

fn main() {
    env_logger::init();
    AppBuilder::simple().run(|app, runtime| {
        runtime.block_on(init(&mut app.world));
    });
}

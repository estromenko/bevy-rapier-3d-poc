use bevy::{
    gltf::{Gltf, GltfMesh, GltfNode},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
pub struct GltfAsset(pub Handle<Gltf>);

fn add_colliders_for_gltf(
    mut commands: Commands,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    assets_gltfnode: Res<Assets<GltfNode>>,
    assets_mesh: Res<Assets<Mesh>>,
    mut event_reader: EventReader<AssetEvent<Gltf>>,
) {
    for event in event_reader.read() {
        if let AssetEvent::Added { id } = event {
            if let Some(gltf) = assets_gltf.get(*id) {
                commands.spawn(SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..default()
                });

                for node in gltf.nodes.iter() {
                    let gltf_node = assets_gltfnode.get(node).unwrap();
                    let handle_gltf_mesh = gltf_node.mesh.clone();
                    let gltf_mesh = assets_gltfmesh.get(handle_gltf_mesh.unwrap()).unwrap();
                    for primitive in gltf_mesh.primitives.iter() {
                        let handle_mesh = primitive.mesh.clone();
                        let mesh = assets_mesh.get(handle_mesh).unwrap();

                        commands.spawn((
                            Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh)
                                .unwrap(),
                            Restitution::coefficient(0.),
                        ));
                    }
                }
            }
        }
    }
}

pub struct GltfAutoCollidersPlugin;

impl Plugin for GltfAutoCollidersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_colliders_for_gltf);
    }
}

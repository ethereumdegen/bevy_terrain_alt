use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::prelude::{Assets, BuildChildren, Commands, Entity, Image, Query, Res, Transform, With, Vec3, Component};

use bevy::transform::TransformBundle;
//use bevy_rapier3d::geometry::Collider;
//use bevy_rapier3d::math::Vect;

use crate::prelude::{NodeAtlas, Terrain, TerrainConfig};
use crate::terrain_data::NodeCoordinate;


//a more generic bevy component 
#[derive(Component,Default)]
pub struct HeightmapCollider {
    
    pub heightmap: Vec<Vec<f32>>,
    pub num_rows: usize,
    pub num_cols: usize,
    pub scale: Vec3, 
}

pub(crate) fn create_collider(
    mut commands: Commands,
    mut terrain_query: Query<(Entity, &mut NodeAtlas, &TerrainConfig), With<Terrain>>,
    images: Res<Assets<Image>>,
) {
    for (entity, mut node_atlas, terrain_config) in terrain_query.iter_mut() {
        let mut new_colliders = HashMap::new();
        for (node_id, node) in &node_atlas.nodes {
            let node_coord = NodeCoordinate::from(*node_id);
            if node_coord.lod > 0 || node_atlas.colliders.contains_key(node_id) {
                continue;
            }
            if let Some(attachment) = &node_atlas.data[node.atlas_index as usize]
                ._attachments
                .get(&0)
            {
                if let Some(image) = images.get(&attachment) {
                    let image_size = image.size();
                    
                
                let division_factor = 8;
                
                let mut heightmap:Vec<Vec<f32>>  = Vec::new();
                for y in (0..image_size.y as usize).step_by(division_factor) {
                    
                   let mut row = Vec::new();
                    
                    for x in (0..image_size.x as usize).step_by(division_factor) {
                        let index = (y * image_size.x as usize + x) * 2;
                        let x = image.data[index] as u16 + ((image.data[index + 1] as u16) << 8);
                        let height = x as f32 / u16::MAX as f32 * terrain_config.height;
                        row.push(height);
                    }
                    
                      heightmap.push( row  )
                } 
               
                    
                    let collider = HeightmapCollider {
                        heightmap,
                        num_rows: image_size.x as usize / division_factor,
                        num_cols: image_size.y as usize / division_factor,
                        scale: Vec3::new((image_size.x - 4. / division_factor as f32) as f32, 1., (image_size.y - 4. / division_factor as f32) as f32),
                      };
                    
                    println!("spawn a collider for terrain ! ");
                    let pos_x = (node_coord.x as f32 + 0.5) * (image_size.x - 4.);
                    let pos_z = (node_coord.y as f32 + 0.5) * (image_size.y - 4.);
                    let mut transform = Transform::from_xyz((pos_x) as f32, 0., (pos_z) as f32);
                    transform.rotate_y(-0.5 * PI);
                    let child = commands
                        .spawn(collider)
                        .insert(TransformBundle::from(transform))
                        .id();
                    commands.entity(entity).push_children(&[child]);
                    new_colliders.insert(*node_id, child);
                }
            }
        }
        node_atlas.colliders.extend(new_colliders);
    }
}
#import bevy_terrain::types TerrainViewConfig,TerrainConfig,TileList
 


// view bindings
#import bevy_pbr::mesh_view_bindings view

// terrain view bindings
@group(1) @binding(0)
var<uniform> view_config: TerrainViewConfig;
@group(1) @binding(1)
var quadtree: texture_2d_array<u32>;
@group(1) @binding(2)
var<storage> tiles: TileList;

// terrain bindings
@group(2) @binding(0)
var<uniform> config: TerrainConfig;
@group(2) @binding(1)
var atlas_sampler: sampler;
@group(2) @binding(2)
var height_atlas: texture_2d_array<f32>;
@group(2) @binding(3)
var minmax_atlas: texture_2d_array<f32>;

#import bevy_pbr::mesh_types
#import bevy_pbr::pbr_types

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
 
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

 
#import bevy_terrain::functions FragmentInput
#import bevy_terrain::debug


#ifndef MINMAX
#import bevy_terrain::vertex
#else
#import bevy_terrain::minmax
#endif

#import bevy_terrain::fragment

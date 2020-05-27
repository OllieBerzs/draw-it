// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// BuiltinMeshes - tegne meshes that can be used without extra code

use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;

use super::Id;
use super::Objects;
use crate::device::Device;
use crate::error::Result;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;

pub(crate) struct BuiltinMeshes {
    pub(crate) surface: Id<Mesh>,
    pub(crate) cube: Id<Mesh>,
    pub(crate) sphere: Id<Mesh>,
}

impl BuiltinMeshes {
    pub(crate) fn new(device: &Arc<Device>, objects: &Objects) -> Result<Self> {
        let surface = objects.add_mesh(create_surface(device)?);
        let cube = objects.add_mesh(create_cube(device)?);
        let sphere = objects.add_mesh(create_sphere(device, 2)?);

        Ok(Self {
            surface,
            cube,
            sphere,
        })
    }
}

fn create_surface(device: &Arc<Device>) -> Result<Mesh> {
    let vertices = &[
        Vector3::new(-1.0, 1.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
    ];
    let uvs = &[
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
    ];
    let triangles = &[[0, 2, 1], [0, 3, 2]];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            triangles,
            uvs,
            ..Default::default()
        },
    )
}

fn create_cube(device: &Arc<Device>) -> Result<Mesh> {
    let vertices = &[
        // bottom
        Vector3::new(-0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, 0.5),
        Vector3::new(-0.5, -0.5, 0.5),
        // top
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(-0.5, 0.5, 0.5),
    ];
    let uvs = &[
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
    ];
    let triangles = &[
        [0, 1, 2],
        [0, 2, 3], // bottom
        [4, 7, 6],
        [4, 6, 5], // top
        [4, 5, 1],
        [4, 1, 0], // front
        [7, 3, 2],
        [7, 2, 6], // back
        [5, 6, 2],
        [5, 2, 1], // right
        [7, 4, 0],
        [7, 0, 3], // left
    ];

    Mesh::new(
        device,
        MeshOptions {
            vertices,
            triangles,
            uvs,
            ..Default::default()
        },
    )
}

fn create_sphere(device: &Arc<Device>, detail_level: u32) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut triangles = vec![];

    // 12 icosahedron vertices
    let t = (1.0 + 5.0f32.sqrt()) / 2.0;

    vertices.push(Vector3::new(-1.0, t, 0.0).unit());
    vertices.push(Vector3::new(1.0, t, 0.0).unit());
    vertices.push(Vector3::new(-1.0, -t, 0.0).unit());
    vertices.push(Vector3::new(1.0, -t, 0.0).unit());

    vertices.push(Vector3::new(0.0, -1.0, t).unit());
    vertices.push(Vector3::new(0.0, 1.0, t).unit());
    vertices.push(Vector3::new(0.0, -1.0, -t).unit());
    vertices.push(Vector3::new(0.0, 1.0, -t).unit());

    vertices.push(Vector3::new(t, 0.0, -1.0).unit());
    vertices.push(Vector3::new(t, 0.0, 1.0).unit());
    vertices.push(Vector3::new(-t, 0.0, -1.0).unit());
    vertices.push(Vector3::new(-t, 0.0, 1.0).unit());

    // 20 icosahedron triangles
    triangles.push([0, 11, 5]);
    triangles.push([0, 5, 1]);
    triangles.push([0, 1, 7]);
    triangles.push([0, 7, 10]);
    triangles.push([0, 10, 11]);

    triangles.push([1, 5, 9]);
    triangles.push([5, 11, 4]);
    triangles.push([11, 10, 2]);
    triangles.push([10, 7, 6]);
    triangles.push([7, 1, 8]);

    triangles.push([3, 9, 4]);
    triangles.push([3, 4, 2]);
    triangles.push([3, 2, 6]);
    triangles.push([3, 6, 8]);
    triangles.push([3, 8, 9]);

    triangles.push([4, 9, 5]);
    triangles.push([2, 4, 11]);
    triangles.push([6, 2, 10]);
    triangles.push([8, 6, 7]);
    triangles.push([9, 8, 1]);

    // refine triangles
    let mut midpoints = HashMap::new();
    for _ in 0..detail_level {
        let mut new_triangles = vec![];
        for tri in triangles {
            // replace triangle with 4 triangles
            let a = get_middle_point(&mut vertices, tri[0], tri[1], &mut midpoints);
            let b = get_middle_point(&mut vertices, tri[1], tri[2], &mut midpoints);
            let c = get_middle_point(&mut vertices, tri[2], tri[0], &mut midpoints);

            new_triangles.push([tri[0], a, c]);
            new_triangles.push([tri[1], b, a]);
            new_triangles.push([tri[2], c, b]);
            new_triangles.push([a, b, c]);
        }
        triangles = new_triangles;
    }

    let mut uvs = vec![];
    for vertex in vertices.iter() {
        let u = vertex.z.atan2(vertex.x) / (2.0 * PI);
        let v = (vertex.y.asin() / PI) + 0.5;
        uvs.push(Vector2::new(u, v));
    }

    Mesh::new(
        device,
        MeshOptions {
            vertices: &vertices,
            triangles: &triangles,
            uvs: &uvs,
            ..Default::default()
        },
    )
}

fn get_middle_point(
    vertices: &mut Vec<Vector3>,
    p1: u32,
    p2: u32,
    midpoints: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    match (midpoints.get(&(p1, p2)), midpoints.get(&(p2, p1))) {
        (Some(i), _) => *i,
        (_, Some(i)) => *i,
        (None, None) => {
            let point_1 = vertices[p1 as usize];
            let point_2 = vertices[p2 as usize];
            let middle = (point_1 + point_2) / 2.0;

            vertices.push(middle.unit());
            let i = vertices.len() as u32 - 1;
            midpoints.insert((p1, p2), i);
            i
        }
    }
}

use tegne::Material;
use tegne::Mesh;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Floor {
    mesh: Mesh,
    material: Material,
    _texture: Texture,
}

impl Floor {
    pub fn new(tegne: &Tegne) -> Self {
        let mesh = plane(tegne, 100.0);
        let texture = tegne.create_texture_from_file("example/assets/prototype_512x512_grey2.png");
        let material = tegne.create_material().with_albedo(&texture).build();

        Self {
            mesh,
            material,
            _texture: texture,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_material(&self.material);
        target.draw(&self.mesh, Transform::default());
    }
}

fn plane(tegne: &Tegne, size: f32) -> Mesh {
    let half_size = size / 2.0;
    let vertices = vec![
        Vector3::new(-half_size, 0.0, half_size),
        Vector3::new(half_size, 0.0, half_size),
        Vector3::new(half_size, 0.0, -half_size),
        Vector3::new(-half_size, 0.0, -half_size),
    ];
    let uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(size, 0.0),
        Vector2::new(size, size),
        Vector2::new(0.0, size),
    ];
    let triangles = vec![0, 1, 2, 0, 2, 3];

    tegne
        .create_mesh()
        .with_vertices(&vertices)
        .with_uvs(&uvs)
        .with_triangles(&triangles)
        .with_smooth_normals()
        .build()
}

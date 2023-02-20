use ambient_std::mesh::Mesh;
use glam::*;

#[derive(Debug, Clone)]
pub struct GridMesh {
    pub top_left: glam::Vec2,
    pub size: glam::Vec2,
    pub n_vertices_width: usize,
    pub n_vertices_height: usize,
    pub uv_min: glam::Vec2,
    pub uv_max: glam::Vec2,
    pub normal: glam::Vec3,
}
impl Default for GridMesh {
    fn default() -> GridMesh {
        GridMesh {
            top_left: glam::Vec2::ZERO,
            size: glam::Vec2::ONE,
            n_vertices_width: 2,
            n_vertices_height: 2,
            uv_min: glam::Vec2::ZERO,
            uv_max: glam::Vec2::ONE,
            normal: glam::Vec3::Z,
        }
    }
}
impl From<GridMesh> for Mesh {
    fn from(box3: GridMesh) -> Mesh {
        From::from(&box3)
    }
}
impl From<&GridMesh> for Mesh {
    fn from(grid: &GridMesh) -> Mesh {
        let mut positions = Vec::new();
        let mut texcoords = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        for y in 0..grid.n_vertices_height {
            for x in 0..grid.n_vertices_width {
                let p = glam::Vec2::new(x as f32 / (grid.n_vertices_width as f32 - 1.0), y as f32 / (grid.n_vertices_height as f32 - 1.0));
                positions.push(vec3(grid.top_left.x + grid.size.x * p.x, grid.top_left.y + grid.size.y * p.y, 0.));
                texcoords.push(vec2(
                    grid.uv_min.x + (grid.uv_max.x - grid.uv_min.x) * p.x,
                    grid.uv_min.y + (grid.uv_max.y - grid.uv_min.y) * p.y,
                ));
                let normal = grid.normal;
                normals.push(vec3(normal.x, normal.y, normal.z));
                if y < grid.n_vertices_height - 1 && x < grid.n_vertices_width - 1 {
                    let vert_index = x + y * grid.n_vertices_width;
                    indices.push((vert_index) as u32);
                    indices.push((vert_index + 1) as u32);
                    indices.push((vert_index + grid.n_vertices_width) as u32);

                    indices.push((vert_index + 1) as u32);
                    indices.push((vert_index + grid.n_vertices_width + 1) as u32);
                    indices.push((vert_index + grid.n_vertices_width) as u32);
                }
            }
        }
        let mut mesh = Mesh {
            name: format!("Grid {}x{}", grid.n_vertices_height, grid.n_vertices_height),
            positions: Some(positions),
            texcoords: vec![texcoords],
            colors: None,
            normals: Some(normals),
            tangents: None,
            indices: Some(indices),
            joint_weights: None,
            joint_indices: None,
        };
        mesh.create_tangents();
        mesh
    }
}

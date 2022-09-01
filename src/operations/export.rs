//! See [Mesh](crate::mesh::Mesh).

use crate::prelude::*;

///
/// # Export
///
/// Methods for extracting raw mesh data which for example can be used for visualisation.
///
impl Mesh {
    ///
    /// Returns the face indices in an array `(i0, i1, i2) = (indices[3*x], indices[3*x+1], indices[3*x+2])` which is meant to be used for visualisation.
    /// Use the `positions_buffer` method and `normals_buffer` method to get the positions and normals of the vertices.
    ///
    pub fn indices_buffer(&self) -> Vec<u32> {
        let vertices: Vec<VertexID> = self.vertex_iter().collect();
        let mut indices = Vec::with_capacity(self.no_faces() * 3);
        for face_id in self.face_iter() {
            for halfedge_id in self.face_halfedge_iter(face_id) {
                let vertex_id = self.walker_from_halfedge(halfedge_id).vertex_id().unwrap();
                let index = vertices.iter().position(|v| v == &vertex_id).unwrap();
                indices.push(index as u32);
            }
        }
        indices
    }

    ///
    /// Returns the positions of the vertices in an array which is meant to be used for visualisation.
    ///
    /// **Note:** The connectivity of the vertices are attained by the `indices_buffer` method.
    ///
    pub fn positions_buffer(&self) -> Vec<Vector3<f64>> {
        self.vertex_iter()
            .map(|vertex_id| self.vertex_position(vertex_id))
            .collect::<Vec<_>>()
    }

    ///
    /// Returns the normals of the vertices in an array which is meant to be used for visualisation.
    ///
    /// **Note:** The connectivity of the vertices are attained by the `indices_buffer` method.
    ///
    /// **Note:** The normal of a vertex is computed as the average of the normals of the adjacent faces.
    ///
    /// **Note:** The normals are computed from the connectivity and positions each time this method is invoked.
    ///
    pub fn normals_buffer(&self) -> Vec<Vector3<f64>> {
        self.vertex_iter()
            .map(|vertex_id| self.vertex_normal(vertex_id))
            .collect::<Vec<_>>()
    }

    ///
    /// Returns the positions of the face corners in an array which is meant to be used for visualisation.
    ///
    pub fn non_indexed_positions_buffer(&self) -> Vec<f64> {
        let mut positions = Vec::with_capacity(self.no_faces() * 3 * 3);
        for face_id in self.face_iter() {
            let (p0, p1, p2) = self.face_positions(face_id);
            push_vec3(&mut positions, p0);
            push_vec3(&mut positions, p1);
            push_vec3(&mut positions, p2);
        }
        positions
    }

    ///
    /// Returns the normals of the face corners in an array which is meant to be used for visualisation.
    ///
    /// **Note:** The normal of a vertex is computed as the average of the normals of the adjacent faces.
    ///
    /// **Note:** The normals are computed from the connectivity and positions each time this method is invoked.
    ///
    pub fn non_indexed_normals_buffer(&self) -> Vec<f64> {
        let mut normals = Vec::with_capacity(self.no_faces() * 3 * 3);
        for face_id in self.face_iter() {
            let (v0, v1, v2) = self.face_vertices(face_id);
            push_vec3(&mut normals, self.vertex_normal(v0));
            push_vec3(&mut normals, self.vertex_normal(v1));
            push_vec3(&mut normals, self.vertex_normal(v2));
        }
        normals
    }
}

fn push_vec3(vec: &mut Vec<f64>, vec3: Vec3) {
    for i in 0..3 {
        vec.push(vec3[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_export() {
        let mesh: Mesh = RawMesh::cylinder(16).into();
        let indices = mesh.indices_buffer();
        let positions = mesh.positions_buffer();
        let normals = mesh.normals_buffer();

        assert_eq!(indices.len(), mesh.no_faces() * 3);
        assert_eq!(positions.len(), mesh.no_vertices());
        assert_eq!(normals.len(), mesh.no_vertices());

        for face in 0..positions.len() / 3 {
            let vertices = (
                indices[3 * face] as usize,
                indices[3 * face + 1] as usize,
                indices[3 * face + 2] as usize,
            );
            let p0 = positions[vertices.0];
            let p1 = positions[vertices.1];
            let p2 = positions[vertices.2];
            let center = (p0 + p1 + p2) / 3.0;
            let face_id = mesh
                .face_iter()
                .find(|face_id| (mesh.face_center(*face_id) - center).magnitude() < 0.00001);
            assert!(face_id.is_some());

            let n0 = normals[vertices.0];
            let n1 = normals[vertices.1];
            let n2 = normals[vertices.2];

            let (v0, v1, v2) = mesh.face_vertices(face_id.unwrap());

            assert!(
                n0 == mesh.vertex_normal(v0)
                    || n1 == mesh.vertex_normal(v0)
                    || n2 == mesh.vertex_normal(v0)
            );
            assert!(
                n0 == mesh.vertex_normal(v1)
                    || n1 == mesh.vertex_normal(v1)
                    || n2 == mesh.vertex_normal(v1)
            );
            assert!(
                n0 == mesh.vertex_normal(v2)
                    || n1 == mesh.vertex_normal(v2)
                    || n2 == mesh.vertex_normal(v2)
            );
        }
    }

    #[test]
    fn test_non_indexed_export() {
        let mesh: Mesh = RawMesh::cylinder(16).into();
        let positions = mesh.non_indexed_positions_buffer();
        let normals = mesh.non_indexed_normals_buffer();

        assert_eq!(positions.len(), mesh.no_faces() * 3 * 3);
        assert_eq!(normals.len(), mesh.no_faces() * 3 * 3);

        for face in 0..positions.len() / 9 {
            let vertices = (9 * face, 9 * face + 3, 9 * face + 6);
            let p0 = vec3(
                positions[vertices.0],
                positions[vertices.0 + 1],
                positions[vertices.0 + 2],
            );
            let p1 = vec3(
                positions[vertices.1],
                positions[vertices.1 + 1],
                positions[vertices.1 + 2],
            );
            let p2 = vec3(
                positions[vertices.2],
                positions[vertices.2 + 1],
                positions[vertices.2 + 2],
            );
            let center = (p0 + p1 + p2) / 3.0;

            let face_id = mesh
                .face_iter()
                .find(|face_id| (mesh.face_center(*face_id) - center).magnitude() < 0.00001);
            assert!(face_id.is_some());

            let n0 = vec3(
                normals[vertices.0],
                normals[vertices.0 + 1],
                normals[vertices.0 + 2],
            );
            let n1 = vec3(
                normals[vertices.1],
                normals[vertices.1 + 1],
                normals[vertices.1 + 2],
            );
            let n2 = vec3(
                normals[vertices.2],
                normals[vertices.2 + 1],
                normals[vertices.2 + 2],
            );

            let (v0, v1, v2) = mesh.face_vertices(face_id.unwrap());

            assert!(
                n0 == mesh.vertex_normal(v0)
                    || n1 == mesh.vertex_normal(v0)
                    || n2 == mesh.vertex_normal(v0)
            );
            assert!(
                n0 == mesh.vertex_normal(v1)
                    || n1 == mesh.vertex_normal(v1)
                    || n2 == mesh.vertex_normal(v1)
            );
            assert!(
                n0 == mesh.vertex_normal(v2)
                    || n1 == mesh.vertex_normal(v2)
                    || n2 == mesh.vertex_normal(v2)
            );
        }
    }
}

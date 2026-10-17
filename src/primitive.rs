use crate::vertex::Vertex;

pub fn primitive_assembly(transformed_vertices: &[Vertex]) -> Vec<[Vertex; 3]> {
    assert!(
        transformed_vertices.len() % 3 == 0,
        "La cantidad de vértices debe ser múltiplo de 3",
    );

    transformed_vertices
        .chunks_exact(3)
        .map(|group| [group[0].clone(), group[1].clone(), group[2].clone()])
        .collect()
}

#[cfg(test)]
mod tests {
    use raylib::prelude::Vector3;

    use super::primitive_assembly;
    use crate::vertex::Vertex;

    fn create_vertices(amount: usize) -> Vec<Vertex> {
        (0..amount)
            .map(|index| {
                Vertex::new_with_color(
                    Vector3::new(index as f32, 0.0, 0.0),
                    Vector3::new(1.0, 1.0, 1.0),
                )
            })
            .collect()
    }

    #[test]
    fn assembles_two_triangles_in_order() {
        let vertices = create_vertices(6);

        let triangles = primitive_assembly(&vertices);

        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0][0].position.x, 0.0);
        assert_eq!(triangles[0][2].position.x, 2.0);
        assert_eq!(triangles[1][0].position.x, 3.0);
        assert_eq!(triangles[1][2].position.x, 5.0);
    }

    #[test]
    #[should_panic(expected = "La cantidad de vértices debe ser múltiplo de 3")]
    fn rejects_an_incomplete_triangle() {
        let vertices = create_vertices(7);

        primitive_assembly(&vertices);
    }
}

use crate::element::Element;
use crate::mesh::Mesh;
use crate::scene::Scene;

/*impl Scene {
    /// Modifies [Scene] to have deduplicated [Mesh]es.
    fn deduplicate_meshes(mut self, tolerance: f64) {
        let deduplication_result = Mesh::deduplicate_with_id_info(self.meshes, tolerance);

        let number_of_meshes = deduplication_result.len();
        let mut new_meshes: Vec<Mesh> = Vec::with_capacity(number_of_meshes);

        for i in 0..number_of_meshes {
            let current = &deduplication_result[i].0;
            let clone = Mesh::new_with_id(current.id, current.coordinates.clone(), current.indices.clone());
            new_meshes.push(clone);
        }
        
        self.meshes = new_meshes;

        let number_of_elements = self.elements.len();
        let mut new_elements: Vec<Element> = Vec::with_capacity(number_of_elements);
        
        
        
        self.elements = new_elements;
    }
}*/
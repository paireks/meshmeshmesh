use std::collections::HashSet;
use crate::scene::Scene;

impl Scene {
    /// Gets all `id`s of the [Mesh]es for which any [Element] applied `face_colors`.
    /// 
    /// This method can be useful for additional checks.
    /// 
    /// # Example
    /// 
    /// In this example we load the [Scene] which has 2 Meshes with `face_colors`
    /// applied (id: 0 and id: 2). That's why we expect 0 and 2 in the output `HashSet`.
    pub (crate) fn get_mesh_ids_with_face_colors_applied(&self) -> HashSet<usize> {
        let mut set: HashSet<usize> = HashSet::new();

        for element in &self.elements {
            if element.face_colors.is_some() { 
                set.insert(element.mesh_id);
            }
        }
        
        set
    }
}
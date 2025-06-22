use std::collections::HashSet;
use crate::scene::Scene;

impl Scene {
    /// Gets all `id`s of the [Mesh]es for which any [Element] applied `face_colors`.
    /// 
    /// This method can be useful for additional checks.
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
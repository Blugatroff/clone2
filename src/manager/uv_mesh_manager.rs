use finger_paint_wgpu::uv_mesh::UvMesh;
use finger_paint_wgpu::Transform;

pub struct UvMeshManager {
    models: Vec<Option<UvMesh>>,
    first_empty: usize,
}
impl UvMeshManager {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            first_empty: 0,
        }
    }
    pub fn insert(&mut self, mesh: UvMesh) -> EcsUvMesh {
        self.models.insert(self.first_empty, Some(mesh));
        let index = self.first_empty;
        self.first_empty = {
            let mut i = self.models.len();
            for (j, slot) in self.models.iter().enumerate() {
                if slot.is_none() {
                    i = j;
                    break;
                }
            }
            i
        };
        EcsUvMesh { index }
    }
    pub fn remove(&mut self, handle: EcsUvMesh) {
        if let Some(mesh) = self.models[handle.index].take() {
            drop(mesh)
        }
    }
    pub fn get(&mut self, handle: &EcsUvMesh) -> Option<&mut UvMesh> {
        self.models[handle.index].as_mut()
    }
    pub fn get_instances<'b>(&'b mut self, handle: &EcsUvMesh) -> Option<&'b mut Vec<Transform>> {
        self.models[handle.index]
            .as_mut()
            .map(move |model| &mut model.instances)
    }
    pub fn get_all_mut(&mut self) -> impl Iterator<Item = &mut UvMesh> {
        self.models.iter_mut().flatten()
    }
    pub fn get_all(&self) -> impl Iterator<Item = &UvMesh> + Clone {
        self.models.iter().flatten()
    }
}

#[derive(Clone)]
pub struct EcsUvMesh {
    index: usize,
}

impl Default for UvMeshManager {
    fn default() -> Self {
        Self::new()
    }
}

use finger_paint_wgpu::model::Model;
use finger_paint_wgpu::Transform;

pub struct ModelManager {
    models: Vec<Option<Model>>,
    first_empty: usize,
}
impl ModelManager {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            first_empty: 0,
        }
    }
    pub fn insert(&mut self, model: Model) -> EcsModelHandle {
        self.models.insert(self.first_empty, Some(model));
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
        EcsModelHandle { index }
    }
    pub fn remove(&mut self, handle: EcsModelHandle) {
        if let Some(model) = self.models[handle.index].take() {
            drop(model)
        }
    }
    pub fn get_instances(&mut self, handle: &EcsModelHandle) -> Option<&mut Vec<Transform>> {
        self.get(handle).map(|model| &mut model.instances)
    }
    pub fn get(&mut self, handle: &EcsModelHandle) -> Option<&mut Model> {
        self.models[handle.index].as_mut()
    }
    pub fn get_all(&self) -> impl Iterator<Item = &Model> + Clone {
        self.models.iter().flatten()
    }
    pub fn get_all_mut(&mut self) -> impl Iterator<Item = &mut Model> {
        self.models.iter_mut().flatten()
    }
}

#[derive(Clone, Copy)]
pub struct EcsModelHandle {
    index: usize,
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}

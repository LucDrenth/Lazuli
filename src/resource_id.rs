use std::marker::PhantomData;

pub struct ResourceId<T> {
    id: u32,
    _phantom: PhantomData<T>,
}

impl<T> ResourceId<T> {
    pub fn new(id: u32) -> Self {
        ResourceId {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn duplicate(&self) -> ResourceId<T> {
        Self::new(self.id)
    }

    /// Compare id's
    pub fn equals(&self, compare_with: &ResourceId<T>) -> bool {
        self.id == compare_with.id
    }
}

use std::marker::PhantomData;

// We wrap the u32 id's in a struct instead of directly using a u32 in the entries so that 
// we can not accidentally try to get an entry with an id from the wrong type

pub struct AssetId<T> {
    id: u32,
    _phantom: PhantomData<T>,
}

impl<T> AssetId<T> {
    pub fn new(id: u32) -> Self {
        AssetId {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn duplicate(&self) -> AssetId<T> {
        Self::new(self.id)
    }
}

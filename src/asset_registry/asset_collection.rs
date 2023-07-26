use std::collections::HashMap;

use super::AssetId;

pub struct AssetEntry<T, U> {
    pub asset: T,
    pub builder_hash: U,
}

/// 'T' is the asset type, like 'Texture', 'Material' etc.
/// 
/// 'U' is the asset its builder hash type, which is a hash of the asset creation data to prevent duplicate entries.
pub struct AssetCollection<T, U: std::cmp::PartialEq> {
    entries: HashMap<u32, AssetEntry<T, U>>,
    id_counter: u32,
}

impl<T, U: std::cmp::PartialEq> AssetCollection<T, U> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn get_by_builder_hash(&self, builder_hash: &U) -> Option<AssetId<T>>{
        for (asset_id, asset_entry) in self.entries.iter() {
            if asset_entry.builder_hash == *builder_hash {
                return Some(AssetId::new(*asset_id))
            }
        }

        None
    }

    pub fn add(&mut self, asset: T, builder_hash: U) -> Result<AssetId<T>, String> {
        self.id_counter += 1;

        match self.entries.entry(self.id_counter) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(format!(
                    "Encountered duplicate id {} while adding asset to asset collection"
                    , self.id_counter
                ));
            },
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(AssetEntry { asset, builder_hash });
            },
        }

        Ok(AssetId::new(self.id_counter))
    }

    pub fn get_asset_by_id(&mut self, id: &AssetId<T>) -> Option<&T> {
        match self.entries.get(id.id()) {
            Some(entry) => Some(&entry.asset),
            None => None,
        }
    }

    pub fn get_mut_asset_by_id(&mut self, id: &AssetId<T>) -> Option<&mut T> {
        match self.entries.get_mut(id.id()) {
            Some(entry) => Some(&mut entry.asset),
            None => None,
        }
    }

    pub fn entries(&self) -> &HashMap<u32, AssetEntry<T, U>> {
        &self.entries
    }
}

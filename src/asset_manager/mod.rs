mod asset_manager;
pub use asset_manager::AssetManager;
pub use asset_manager::GlAssetManager;

mod asset_collection;
pub use asset_collection::AssetCollection;


pub mod asset_manager_mock;

#[cfg(test)]
mod asset_collection_test;

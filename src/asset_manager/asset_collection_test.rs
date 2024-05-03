use crate::ResourceId;

use super::AssetCollection;

#[test]
fn test_add() {
    let mut asset_collection: AssetCollection<&str, usize> = AssetCollection::new();

    assert!(asset_collection.add("", 0).is_ok());
    assert!(asset_collection.add("", 0).is_ok());
}

#[test]
fn test_get_by_builder_hash() -> Result<(), String> {
    let mut asset_collection: AssetCollection<&str, usize> = AssetCollection::new();
    
    let builder_hash = 1;
    let non_existing_builder_hash = 2;

    _ = asset_collection.add("", builder_hash)?;

    assert!(asset_collection.get_by_builder_hash(&builder_hash).is_some());
    assert!(asset_collection.get_by_builder_hash(&non_existing_builder_hash).is_none());

    Ok(())
}

#[test]
fn test_get_asset_by_id() -> Result<(), String> {
    let mut asset_collection: AssetCollection<&str, usize> = AssetCollection::new();

    let asset_id = asset_collection.add("", 0)?;
    let non_existing_id: ResourceId<&str> = ResourceId::new(asset_id.id() + 1);

    assert!(asset_collection.get_asset_by_id(&asset_id).is_some());
    assert!(asset_collection.get_asset_by_id(&non_existing_id).is_none());

    assert!(asset_collection.get_mut_asset_by_id(&asset_id).is_some());
    assert!(asset_collection.get_mut_asset_by_id(&non_existing_id).is_none());

    Ok(())
}

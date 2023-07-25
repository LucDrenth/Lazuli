pub struct AssetEntry<T, U> {
    pub asset: T,
    /// a hash with data that is used to create the asset, used to prevent duplicate entries
    pub builder_hash: U,
}

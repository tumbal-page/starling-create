fn create_assets_root(result: std::io::Result<std::path::PathBuf>) -> std::io::Result<std::path::PathBuf> {
    result.map(|root| crate::game::create_port::asset_root(root))
}

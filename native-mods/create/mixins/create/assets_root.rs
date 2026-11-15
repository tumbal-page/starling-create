fn create_assets_root(result: &mut std::io::Result<std::path::PathBuf>) {
    *result = result.as_ref().ok().map(|root| crate::game::create_port::asset_root(root.clone()));
}

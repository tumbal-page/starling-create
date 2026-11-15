//! Native bundle source enumeration, content snapshots and installed staging.
use super::*;

/// The root of an unpacked combined mod, or the mixin folder for code-only mods.
fn bundle_root(bundle: &Bundle) -> &Path {
    bundle
        .dir
        .parent()
        .and_then(Path::parent)
        .filter(|root| {
            root.join(NATIVE_MANIFEST).is_file()
                && bundle.dir == root.join("mixins").join(&bundle.id)
        })
        .unwrap_or(&bundle.dir)
}

pub(super) fn content_files(bundle: &Bundle) -> Result<Vec<(String, PathBuf)>, String> {
    let root = bundle_root(bundle);
    if !root.join(horizon_mods::manifest::MANIFEST).is_file() {
        return Ok(Vec::new());
    }
    let package = horizon_mods::Package::read(root)?;
    if package.manifest.id != bundle.id {
        return Err(tr("Native and content mod ids differ.").into());
    }
    let content = horizon_mods::Content::read(root, &package.manifest)?;
    Ok(std::iter::once(horizon_mods::manifest::MANIFEST.to_owned())
        .chain(content.files)
        .map(|name| {
            let path = root.join(&name);
            (name, path)
        })
        .collect())
}

pub(super) fn build_files(bundle: &Bundle) -> Result<Vec<(String, PathBuf)>, String> {
    let mut files = bundle_files(bundle)?;
    files.extend(content_files(bundle)?);
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

/// Content is owned by this exact build, never read from a later installed mod update.
pub(super) fn install_content(mods: &[Bundle], version: &Path) -> Result<(), String> {
    for bundle in mods {
        for (name, path) in content_files(bundle)? {
            let to = version.join("native-content").join(&bundle.id).join(name);
            fs::create_dir_all(to.parent().unwrap()).map_err(|e| e.to_string())?;
            fs::copy(path, to).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub(super) fn installed_mods_into(home: &Path, out: &Path) -> Result<PathBuf, String> {
    let _ = fs::remove_dir_all(out);
    let mut paths: Vec<PathBuf> = fs::read_dir(home.join(INSTALLED))
        .map(|dir| dir.flatten().map(|e| e.path()).collect())
        .unwrap_or_default();
    paths.retain(|p| horizon_mods::index::native::is_file(p));
    paths.sort();
    let mut count = 0;
    let mut seen = std::collections::BTreeSet::new();
    for path in paths {
        let bundle = horizon_mods::index::native::Bundle::read(&path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        if !crate::browser::java::is_enabled(home, &bundle.manifest.id) {
            continue;
        }
        if !seen.insert(bundle.manifest.id.clone()) {
            return Err(trf(
                "Duplicate native mod {id}.",
                &[("id", &bundle.manifest.id)],
            ));
        }
        let dir = out.join(&bundle.manifest.id);
        for (name, bytes) in bundle.source_files() {
            let to = dir.join(name);
            fs::create_dir_all(to.parent().unwrap_or(&dir)).map_err(|e| e.to_string())?;
            fs::write(&to, bytes).map_err(|e| format!("{}: {e}", to.display()))?;
        }
        count += 1;
    }
    if count == 0 {
        return Err(format!(
            "No native mod is turned on in {}: install one from the Mods page first.",
            home.join(INSTALLED).display()
        ));
    }
    Ok(out.to_path_buf())
}

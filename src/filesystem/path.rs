use crate::{
    error::{Error, Result},
    model::{config::AppConfig, ResizeOptions},
};
use std::env::current_dir;
use std::path::{Path, PathBuf};

/// Gets the current directory with a trailling slash
fn get_current_dir() -> Result<PathBuf> {
    let dir = current_dir()?;

    // Dumb way to add trailing slash, need trailing slash added so the file
    // path is propperly stripped
    let mut dir_str = dir.into_os_string();

    if !dir_str.to_string_lossy().ends_with('/') {
        dir_str.push("/");
    }

    Ok(PathBuf::from(dir_str))
}

fn get_relative_dir<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let cur_dir = get_current_dir()?;

    // Watcher paths prepend absolute paths, so remove it here
    // eg: /mnt/c/.../picatch/./photos/DSC_3328.jpg
    // We only want the ./photos/DSC_3328.jpg part. We don't need to
    // canonicalize() path beforehand since doesn't matter if there's an extra
    // /./ in the middle the /mnt/c/.../picatch/ part will be stripped
    let path = path.as_ref().strip_prefix(cur_dir).unwrap_or({
        // If stripping cur_dir doesn't work, there's still the "./"
        path.as_ref().strip_prefix(".").unwrap_or(path.as_ref())
    });

    Ok(path.to_path_buf())
}

/// Gets the directory of a file.  If a directory is passed in, then returns the directory
fn get_file_dir<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    if path.is_dir() {
        return Ok(path.into());
    }

    // If is file, get the parent dir. If dir, don't get parent
    // This contains original_photos_dir/path/to/dir
    let extension = match path.extension() {
        Some(ext) => ext.to_string_lossy().to_lowercase(),
        None => return Ok(path.into()),
    };

    if extension != "jpg" && extension != "jpeg" {
        return Ok(path.into());
    }

    let dir = path.parent().ok_or(Error::Picatch(format!(
        "Path missing parent: {}",
        path.to_string_lossy()
    )))?;

    Ok(dir.into())
}

// Converts path in original_photos_dir to a path in resized_photos_dir
pub fn get_resized_dir_path(config: &AppConfig, path: &Path) -> Result<PathBuf> {
    let path_str = path.to_string_lossy();

    // Assumes config paths do NOT have the "./" prefix
    // Should be handled when creating the config
    let original_photos_dir = PathBuf::from(&config.original_photos_dir);

    // Resized dir + relative file path + size
    let mut dest_path = PathBuf::from(&config.resized_photos_dir);

    let relative_path = get_relative_dir(path)?;

    let source_file_dir = get_file_dir(&relative_path)?;

    // Path is relative, so we need to handle absolute paths for original_photos_dir

    // This is the path to dir *without* the original_photos_dir
    let file_dir = source_file_dir
        .strip_prefix(&original_photos_dir)
        .map_err(|_| {
            Error::Picatch(format!(
                "get_resized_dir_path: Failed to strip original_photos_dir ({}) from source_file_dir ({}), path: {}",
                original_photos_dir.to_string_lossy(),
                source_file_dir.to_string_lossy(),
                path_str
            ))
        })?;

    dest_path.push(file_dir);

    Ok(dest_path)
}

// Paths will *not* have "./" prefix
pub fn get_destination_path(
    config: &AppConfig,
    path: &Path,
    opts: &ResizeOptions,
) -> Result<PathBuf> {
    let mut dest_path = get_resized_dir_path(config, path)?;

    let file_name = path
        .file_stem()
        .ok_or(Error::Picatch(format!(
            "Path missing file name: {}",
            path.to_string_lossy()
        )))?
        .to_string_lossy();

    let file_ext = path
        .extension()
        .ok_or(Error::Picatch(format!(
            "Path missing extension: {}",
            path.to_string_lossy()
        )))?
        .to_string_lossy();

    // Create new file name with size attached.
    // Not including hash for now, frontend doesn't know about the hash
    let new_file_name = format!("{}-{}.{}", file_name, opts.name, file_ext);
    dest_path.push(&new_file_name);

    Ok(dest_path)
}

/// Get the resized image paths from a single original image path
pub fn get_resized_paths(
    config: &AppConfig,
    source: &Path,
    options_list: &Vec<ResizeOptions>,
) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for options in options_list {
        let dest = get_destination_path(&config, &source, &options)?;

        paths.push(dest);
    }

    Ok(paths)
}

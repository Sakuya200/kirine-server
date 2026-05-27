use std::env::{current_exe};
use std::fs;
use std::path::PathBuf;
use anyhow::{bail, Context, Result};

const SRC_MODEL_RELATIVE_PATH_LIST: &[&str] = &["./src-model", "./lib/src-model"];
pub fn root_dir_path() -> Result<PathBuf> {
    Ok(
        current_exe()?
            .parent()
            .expect("failed to get parent directory of executable")
            .to_path_buf()
    )
}

pub fn config_file_path() -> Result<PathBuf> {
    Ok(root_dir_path()?.join("config.toml"))
}

pub fn resolve_log_dir(log_dir: &PathBuf) -> Result<PathBuf> {
    if !log_dir.exists() {
        bail!("Log directory does not exist: {:?}", log_dir);
    }

    if log_dir.is_absolute() {
        Ok(log_dir.clone())
    } else {
        Ok(root_dir_path()?.join(log_dir))
    }
}

pub fn resolve_src_model_path() -> Result<PathBuf> {
    for &src_model_path in SRC_MODEL_RELATIVE_PATH_LIST {
        let src_model_path = PathBuf::from(src_model_path);
        if src_model_path.exists() {
            return Ok(root_dir_path()?.join(src_model_path));
        }
    }

    bail!("failed to find src-model directory in any of the following paths: ${{ROOT_PATH}}/{:?}", SRC_MODEL_RELATIVE_PATH_LIST);
}
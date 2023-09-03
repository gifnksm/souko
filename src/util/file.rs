use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read},
};

use color_eyre::eyre::{eyre, Error, Result, WrapErr};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use toml_edit::Document;

use crate::domain::model::path_like::PathLike;

pub(crate) fn open(name: &str, path: &impl PathLike) -> Result<Option<File>> {
    let file = match File::open(path.as_real_path()) {
        Ok(file) => Some(file),
        Err(e) if e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            bail!(Error::from(e).wrap_err(format!("failed to open {name}: {}", path.display())))
        }
    };
    Ok(file)
}

pub(crate) fn load_json<T>(name: &str, path: &impl PathLike) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let file = match open(name, path)? {
        Some(file) => file,
        None => return Ok(None),
    };

    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)
        .wrap_err_with(|| format!("failed to read {name}: {}", path.display()))?;
    Ok(Some(value))
}

pub(crate) fn store_json<T>(name: &str, path: &impl PathLike, value: &T) -> Result<()>
where
    T: Serialize,
{
    let dir = path
        .as_real_path()
        .parent()
        .ok_or_else(|| eyre!("failed to get parent directory: {}", path.display()))?;
    fs::create_dir_all(dir)
        .wrap_err_with(|| format!("failed to create diretory: {}", dir.display()))?;

    let file = NamedTempFile::new_in(dir)
        .wrap_err_with(|| format!("failed to create temporary file in {}", dir.display(),))?;
    let temp_path = file.path().to_owned();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, value).wrap_err_with(|| {
        format!(
            "failed to write {name} to temporary file: {}",
            temp_path.display()
        )
    })?;
    let mut file = writer.into_inner()?;
    file.as_file_mut().sync_all()?;
    file.persist(path.as_real_path())
        .wrap_err_with(|| format!("failed to write {name} to file: {}", path.display()))?;
    Ok(())
}

pub(crate) fn load_toml_document(name: &str, path: &impl PathLike) -> Result<Option<Document>> {
    let mut file = match open(name, path)? {
        Some(file) => file,
        None => return Ok(None),
    };

    let mut toml = String::new();
    file.read_to_string(&mut toml)
        .wrap_err_with(|| format!("failed to read {name}: {}", path.display()))?;

    let doc = toml
        .parse()
        .wrap_err_with(|| format!("failed to parse {name}: {}", path.display()))?;

    Ok(Some(doc))
}

pub(crate) fn load_toml<T>(name: &str, path: &impl PathLike) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let doc = load_toml_document(name, path)?;
    let value = match doc {
        Some(toml) => toml_edit::de::from_document(toml)
            .wrap_err_with(|| format!("failed to parse {name}: {}", path.display()))?,
        None => None,
    };
    Ok(value)
}

use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use color_eyre::eyre::{Error, Result, WrapErr};
use serde::Deserialize;
use toml_edit::Document;

use super::optional_param::OptionalParam;
use crate::domain::model::path_like::PathLike;

pub(crate) fn open(path: &OptionalParam<impl PathLike>) -> Result<Option<File>> {
    let file = match File::open(path.value().as_path()) {
        Ok(file) => Some(file),
        Err(e) if path.is_default() && e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(Error::from(e).wrap_err(format!(
                "failed to open {}: {}",
                path.name(),
                path.value().display(),
            )))
        }
    };
    Ok(file)
}

pub(crate) fn canonicalize(path: &OptionalParam<impl PathLike>) -> Result<Option<PathBuf>> {
    let path = match path.value().as_path().canonicalize() {
        Ok(file) => Some(file),
        Err(e) if path.is_default() && e.kind() == io::ErrorKind::NotFound => None,
        Err(e) => {
            return Err(Error::from(e).wrap_err(format!(
                "failed to get absolute path of {}: {}",
                path.name(),
                path.value().display()
            )))
        }
    };
    Ok(path)
}

// pub(crate) fn load_json<T>(path: &OptionalParam<impl PathLike>) -> Result<Option<T>>
// where
//     T: for<'de> Deserialize<'de>,
// {
//     let file = match open(path)? {
//         Some(file) => file,
//         None => return Ok(None),
//     };

//     let reader = BufReader::new(file);
//     let value = serde_json::from_reader(reader)
//         .wrap_err_with(|| format!("failed to read {}: {}", path.name(), path.value().display()))?;
//     Ok(Some(value))
// }

// pub(crate) fn store_json<T>(path: &OptionalParam<impl PathLike>, value: &T) -> Result<()>
// where
//     T: Serialize,
// {
//     let dir = path
//         .value()
//         .parent()
//         .ok_or_else(|| eyre!("failed to get parent directory: {}", path.value().display()))?;
//     fs::create_dir_all(dir)
//         .wrap_err_with(|| format!("failed to create diretory: {}", dir.display()))?;

//     let file = NamedTempFile::new_in(dir)
//         .wrap_err_with(|| format!("failed to create temporary file in {}", dir.display(),))?;
//     let temp_path = file.path().to_owned();
//     let mut writer = BufWriter::new(file);
//     serde_json::to_writer(&mut writer, value).wrap_err_with(|| {
//         format!(
//             "failed to write {} to temporary file: {}",
//             path.name(),
//             temp_path.display()
//         )
//     })?;
//     let mut file = writer.into_inner()?;
//     file.as_file_mut().sync_all()?;
//     file.persist(path).wrap_err_with(|| {
//         format!(
//             "failed to write {} to file: {}",
//             path.name(),
//             path.value().display()
//         )
//     })?;
//     Ok(())
// }

pub(crate) fn load_toml_document(path: &OptionalParam<impl PathLike>) -> Result<Option<Document>> {
    let mut file = match open(path)? {
        Some(file) => file,
        None => return Ok(None),
    };

    let mut toml = String::new();
    file.read_to_string(&mut toml)
        .wrap_err_with(|| format!("failed to read {}: {}", path.name(), path.value().display()))?;

    let doc = toml.parse().wrap_err_with(|| {
        format!(
            "failed to parse {}: {}",
            path.name(),
            path.value().display()
        )
    })?;

    Ok(Some(doc))
}

pub(crate) fn load_toml<T>(path: &OptionalParam<impl PathLike>) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let doc = load_toml_document(path)?;
    let value = match doc {
        Some(toml) => toml_edit::de::from_document(toml).wrap_err_with(|| {
            format!(
                "failed to parse {}: {}",
                path.name(),
                path.value().display()
            )
        })?,
        None => None,
    };
    Ok(value)
}

// pub(crate) fn store_toml_document(path: &OptionalParam<impl PathLike>) -> Result<()> {
//     let dir = path
//         .value()
//         .parent()
//         .ok_or_else(|| eyre!("failed to get parent directory: {}", path.value().display()))?;

//     let file = NamedTempFile::new_in(dir)
//         .wrap_err_with(|| format!("failed to create temporary file in {}", dir.display(),))?;
//     let temp_path = file.path().to_owned();
//     let toml = doc.to_string();
//     let mut writer = BufWriter::new(file);
//     writer.write_all(toml.as_bytes()).wrap_err_with(|| {
//         format!(
//             "failed to write {} to temporary file: {}",
//             path.name(),
//             temp_path.display()
//         )
//     })?;
//     let mut file = writer.into_inner()?;
//     file.as_file_mut().sync_all()?;
//     file.persist(path).wrap_err_with(|| {
//         format!(
//             "failed to write {} to file: {}",
//             path.name(),
//             path.value().display()
//         )
//     })?;
//     Ok(())
// }

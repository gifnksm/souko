use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read},
    path::PathBuf,
};

use color_eyre::eyre::{eyre, Error, Result, WrapErr};
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use toml_edit::Document;

#[derive(Debug)]
enum OptionalParamValue<'a, T> {
    /// Explicitly specified value (via command line argument, environment variable, configuration file, etc.)
    Explicit(&'a T),
    /// Automatically determined value
    Default(T),
}

impl<'a, T> OptionalParamValue<'a, T> {
    fn is_default(&self) -> bool {
        matches!(self, Self::Default(_))
    }

    fn as_ref(&self) -> &T {
        match self {
            Self::Explicit(v) => v,
            Self::Default(v) => v,
        }
    }
}

#[derive(Debug)]
pub(crate) struct OptionalParam<'a, T> {
    name: &'static str,
    value: OptionalParamValue<'a, T>,
}

impl<'a, T> OptionalParam<'a, T> {
    pub(super) fn new(
        name: &'static str,
        value: &'a Option<T>,
        default: impl FnOnce() -> T,
    ) -> Self {
        let value = match value {
            Some(path) => OptionalParamValue::Explicit(path),
            None => OptionalParamValue::Default(default()),
        };
        OptionalParam { name, value }
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn value(&self) -> &T {
        self.value.as_ref()
    }
}

impl<'a> OptionalParam<'a, PathBuf> {
    pub(crate) fn load_json<T>(&self) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let path = self.value.as_ref();

        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) if self.value.is_default() && e.kind() == io::ErrorKind::NotFound => {
                return Ok(None)
            }
            Err(e) => {
                return Err(Error::from(e).wrap_err(format!(
                    "failed to open {}: {}",
                    self.name,
                    path.display()
                )))
            }
        };

        let reader = BufReader::new(file);
        let value = serde_json::from_reader(reader).wrap_err_with(|| {
            format!(
                "failed to read {}: {}",
                self.name,
                self.value.as_ref().display()
            )
        })?;
        Ok(Some(value))
    }

    pub(crate) fn store_json<T>(&self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        let path = self.value.as_ref();

        let dir = path
            .parent()
            .ok_or_else(|| eyre!("failed to get parent directory: {}", path.display()))?;

        let mut file = NamedTempFile::new_in(dir)
            .wrap_err_with(|| format!("failed to create temporary file in {}", dir.display(),))?;
        let temp_path = file.path().to_owned();
        {
            let mut writer = BufWriter::new(&mut file);
            serde_json::to_writer(&mut writer, value).wrap_err_with(|| {
                format!(
                    "failed to write {} to temporary file: {}",
                    self.name,
                    temp_path.display()
                )
            })?;
        }
        file.persist(path).wrap_err_with(|| {
            format!("failed to write {} to file: {}", self.name, path.display())
        })?;
        Ok(())
    }

    pub(crate) fn load_toml_document(&self) -> Result<Option<Document>> {
        let path = self.value.as_ref();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) if self.value.is_default() && e.kind() == io::ErrorKind::NotFound => {
                return Ok(None)
            }
            Err(e) => {
                return Err(Error::from(e).wrap_err(format!(
                    "failed to open {}: {}",
                    self.name,
                    path.display()
                )))
            }
        };

        let mut toml = String::new();
        file.read_to_string(&mut toml)
            .wrap_err_with(|| format!("failed to read P{}: {}", self.name, path.display()))?;

        let doc = toml
            .parse()
            .wrap_err_with(|| format!("failed to parse {}: {}", self.name, path.display()))?;

        Ok(Some(doc))
    }

    pub(crate) fn load_toml<T>(&self) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let doc = self.load_toml_document()?;
        let value = match doc {
            Some(toml) => toml_edit::de::from_document(toml).wrap_err_with(|| {
                format!(
                    "failed to parse {}: {}",
                    self.name(),
                    self.value().display()
                )
            })?,
            None => None,
        };
        Ok(value)
    }

    // pub(crate) fn store_toml_document(&self, doc: &Document) -> Result<()> {
    //     let path = self.value.as_ref();

    //     let dir = path
    //         .parent()
    //         .ok_or_else(|| eyre!("failed to get parent directory: {}", path.display()))?;

    //     let mut file = NamedTempFile::new_in(dir)
    //         .wrap_err_with(|| format!("failed to create temporary file in {}", dir.display(),))?;
    //     let temp_path = file.path().to_owned();
    //     {
    //         let toml = doc.to_string();
    //         let mut writer = BufWriter::new(&mut file);
    //         writer.write_all(toml.as_bytes()).wrap_err_with(|| {
    //             format!(
    //                 "failed to write {} to temporary file: {}",
    //                 self.name,
    //                 temp_path.display()
    //             )
    //         })?;
    //     }
    //     file.persist(path).wrap_err_with(|| {
    //         format!("failed to write {} to file: {}", self.name, path.display())
    //     })?;
    //     Ok(())
    // }
}

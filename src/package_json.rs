use crate::error::CommonError;
use serde::Deserialize;
use std::{collections::HashMap, fs, io::BufReader, path::Path};

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub repository: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
}

impl PackageJson {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, CommonError> {
        let file = fs::File::open(path)?;

        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `PackageJson`.
        let pkg_json: PackageJson = serde_json::from_reader(reader)?;

        Ok(pkg_json)
    }
}

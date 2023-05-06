use crate::error::CommonError;
use serde::Deserialize;
use std::{collections::HashMap, fs, io::BufReader, path::Path};

#[derive(Deserialize, Debug)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub repository: Option<PkgRepo>,
    pub scripts: Option<HashMap<String, String>>,

    #[serde(rename = "packageManager")]
    pub package_manager: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PkgRepo {
    String(String),
    HashMap(HashMap<String, String>),
}

impl PackageJson {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, CommonError> {
        let file = fs::File::open(path)?;

        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `PackageJson`.
        let pkg_json: PackageJson = serde_json::from_reader(reader)?;

        Ok(pkg_json)
    }

    pub fn get_url(&self) -> Result<String, CommonError> {
        match &self.repository {
            Some(repo) => match repo {
                PkgRepo::String(repository) => parse_url(repository),
                PkgRepo::HashMap(repo) => match repo.get("url") {
                    Some(url) => parse_url(url),
                    None => Err(CommonError::NotFound(
                        "package.json repository url field not found!".to_string(),
                    )),
                },
            },
            None => Err(CommonError::NotFound(
                "package.json repository field not found!".to_string(),
            )),
        }
    }
}

fn parse_url(url: &str) -> Result<String, CommonError> {
    let mut url = url.to_string();
    // TODO: use regex
    if url.starts_with("git+") {
        url = url.replace("git+", "");
    }
    if url.ends_with(".git") {
        url = url.replace(".git", "");
    }
    // TODO: validate url
    if url.is_empty() {
        return Err(CommonError::NotFound(
            "package.json repository url field is empty!".to_string(),
        ));
    }
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_package_json() {
        let result = PackageJson::from_path("tests/package.json");
        assert_eq!(result.is_ok(), true);
    }
}

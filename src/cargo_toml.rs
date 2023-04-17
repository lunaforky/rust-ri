use crate::error::CommonError;
use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Deserialize, Debug)]
pub struct CargoToml {
    package: Option<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    homepage: Option<String>,
    repository: Option<String>,
}

impl CargoToml {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, CommonError> {
        let contents = fs::read_to_string(path)?;

        let cargo_toml: CargoToml = toml::from_str(&contents).unwrap();

        Ok(cargo_toml)
    }

    pub fn get_url(&self) -> Result<String, CommonError> {
        match &self.package {
            Some(pkg) => match &pkg.homepage {
                Some(url) => Ok(url.to_string()),
                None => match &pkg.repository {
                    Some(url) => Ok(url.to_string()),
                    None => Err(CommonError::NotFound(
                        "cargo.toml [package] homepage or repository field not found!".to_string(),
                    )),
                },
            },
            None => Err(CommonError::NotFound(
                "cargo.toml [package] field not found!".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_toml() {
        let cargo_toml = CargoToml::from_path("Cargo.toml").unwrap();
        let homepage = cargo_toml.get_url().unwrap();
        assert_eq!(homepage, "https://github.com/JiatLn/ri");
    }
}

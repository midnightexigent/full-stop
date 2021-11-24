use globset::Glob;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub modules: BTreeMap<String, Module>,
    pub deploy: Vec<Deploy>,
}

impl Config {
    fn read_from_file() -> Result<Self, crate::Error> {
        let toml = std::fs::read(".fullstop.toml").map_err(crate::error::ReadConfig::OpenFile)?;
        Ok(toml::from_slice(&toml).map_err(crate::error::ReadConfig::ParseToml)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub include: Vec<Include>,
    pub exclude: Vec<Glob>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Include {
    pub glob: Glob,
    pub tag: Option<Tag>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tag {
    Rename(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deploy {
    pub module: String,
    pub directory: String,
}

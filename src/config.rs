use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "fus.toml";

pub fn read() -> crate::Result<Config> {
    let toml = std::fs::read(CONFIG_FILE)?;
    Ok(toml::from_slice(&toml)?)
}
pub fn init() -> crate::Result<()> {
    let mut modules = Vec::new();
    for result in ignore::WalkBuilder::new("./")
        .max_depth(Some(1))
        .build()
        .filter_map(|entry| {
            entry
                .map(|entry| {
                    if matches!(entry.file_type(), Some(x) if x.is_dir()) {
                        Some(entry)
                    } else {
                        None
                    }
                })
                .transpose()
        })
    {
        let entry = result?;
        let path = entry.path();
        if let (Some(module_name), Some(path)) = (
            path.file_name().and_then(|file_name| file_name.to_str()),
            path.to_str(),
        ) {
            modules.push(Module {
                name: module_name.to_string(),
                includes: vec![path.to_string()],
                destination: format!("$CONFIG_DIR/{}", module_name),
            })
        }
    }
    let toml = toml::to_string_pretty(&Config {
        module: modules,
        vars: toml::Value::Table(toml::map::Map::new()),
    })?;
    Ok(std::fs::write(CONFIG_FILE, toml)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub module: Vec<Module>,
    pub vars: toml::Value,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub destination: String,
    pub includes: Vec<String>,
}

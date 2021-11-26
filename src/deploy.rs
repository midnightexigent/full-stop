use std::path::PathBuf;

pub fn get_env(name: &str) -> crate::Result<Option<String>> {
    let path = match name {
        "CONFIG_DIR" => dirs::config_dir(),
        "DATA_DIR" => dirs::data_dir(),
        "DATA_LOCAL_DIR" => dirs::data_local_dir(),
        "EXECUTABLE_DIR" => dirs::executable_dir(),
        _ => None,
    }
    .and_then(|path| path.to_str().map(ToString::to_string));

    Ok(path)
}

#[derive(Debug)]
pub struct Source {
    pub path: PathBuf,
    pub skip: usize,
}

#[derive(Debug)]
pub struct Deploy {
    pub sources: Vec<Source>,
    pub destination: PathBuf,
}

impl Deploy {
    pub fn from_config(module: crate::config::Module) -> crate::Result<Self> {
        let mut sources = Vec::new();
        for include in module.includes {
            for path in glob::glob(&include.glob)? {
                sources.push(Source {
                    path: path?,
                    skip: include.prefix_strip.unwrap_or(1),
                })
            }
        }

        let destination = PathBuf::from(
            shellexpand::full_with_context(&module.destination, dirs::home_dir, get_env)?
                .to_string(),
        );
        Ok(Self {
            sources,
            destination,
        })
    }
    pub fn copy(&self) -> crate::Result<()> {
        for source in &self.sources {
            let from = &source.path;
            let to = self
                .destination
                .join(from.components().skip(source.skip).collect::<PathBuf>());
            println!("copy from {} to {}", from.display(), to.display())
            // std::fs::copy(from, to)?;
        }
        Ok(())
    }
}

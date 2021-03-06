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
pub struct Deploy {
    pub sources: Vec<PathBuf>,
    pub destination: PathBuf,
}

impl Deploy {
    pub fn from_config(include: crate::config::Include) -> crate::Result<Self> {
        let paths = glob::glob(include.pattern.as_str())?.collect::<Result<Vec<_>, _>>()?;

        let mut sources = Vec::new();
        for path in paths {
            if path.is_dir() {
                let mut files = walkdir::WalkDir::new(path)
                    .into_iter()
                    .filter_map(|e| {
                        e.map(|e| {
                            let e = e.into_path();
                            if e.is_dir() {
                                None
                            } else {
                                Some(e)
                            }
                        })
                        .transpose()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                sources.append(&mut files)
            } else {
                sources.push(path);
            }
        }
        sources.sort();
        sources.dedup();

        let destination = PathBuf::from(
            shellexpand::full_with_context(&include.destination, dirs::home_dir, get_env)?
                .to_string(),
        );
        Ok(Self {
            sources,
            destination,
        })
    }
    pub fn copy(&self) -> crate::Result<()> {
        for source in &self.sources {
            let from = source;
            let to = self
                .destination
                .join(from.components().skip(1).collect::<PathBuf>());
            println!("{} => {}", from.display(), to.display());
            if let Some(to) = to.parent() {
                if !to.exists() {
                    std::fs::create_dir_all(to)?;
                }
            }
            std::fs::copy(from, to)?;
        }
        Ok(())
    }
}

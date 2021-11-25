use camino::Utf8PathBuf;
use relative_path::RelativePathBuf;

pub fn get_env(name: &str) -> crate::Result<Option<Utf8PathBuf>> {
    let path = match name {
        "CONFIG_DIR" => dirs::config_dir(),
        "DATA_DIR" => dirs::data_dir(),
        "DATA_LOCAL_DIR" => dirs::data_local_dir(),
        "EXECUTABLE_DIR" => dirs::executable_dir(),
        _ => None,
    };

    Ok(path.map(Utf8PathBuf::try_from).transpose()?)
}

#[derive(Debug)]
pub struct Deploy {
    pub sources: Vec<RelativePathBuf>,
    pub destination: Utf8PathBuf,
}

impl Deploy {
    pub fn from_config(module: crate::config::Module) -> crate::Result<Self> {
        let mut sources = Vec::new();
        for include in module.includes {
            for path in glob::glob(&include)? {
                sources.push(relative_path::RelativePathBuf::from_path(path?)?)
            }
        }

        let destination = Utf8PathBuf::from(
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
            let from = source.to_path(".");
            let to = self.destination.join(source.as_str());
            println!(
                "copy from {} to {}",
                from.display(),
                to.as_std_path().display()
            )
            // std::fs::copy(from, to)?;
        }
        Ok(())
    }
}

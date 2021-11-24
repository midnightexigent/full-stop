use globset::GlobSetBuilder;
use relative_path::RelativePathBuf;
use walkdir::WalkDir;

pub struct Module {
    name: String,
    paths: Vec<(RelativePathBuf, Vec<crate::config::Tag>)>,
}

impl TryFrom<(crate::config::Config, String)> for Module {
    type Error = crate::Error;

    fn try_from(
        (config, module_name): (crate::config::Config, String),
    ) -> Result<Self, Self::Error> {
        let module = config
            .modules
            .get(&module_name)
            .ok_or_else(|| crate::error::ReadConfig::InvalidModuleName(module_name.clone()))?;
        let include = {
            let mut gsb = GlobSetBuilder::new();
            for entry in &module.include {
                gsb.add(entry.glob.clone());
            }
            gsb.build()?
        };
        let exclude = {
            let mut gsb = GlobSetBuilder::new();
            for entry in &module.exclude {
                gsb.add(entry.clone());
            }
            gsb.build()?
        };
        let paths: Result<Vec<_>, _> = WalkDir::new(".")
            .into_iter()
            .flatten()
            .filter(|entry| include.is_match(entry.path()) && !exclude.is_match(entry.path()))
            .map(|entry| {
                RelativePathBuf::from_path(entry.path()).map(|relative_path| {
                    (
                        relative_path,
                        include
                            .matches(entry.path())
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, num_matches)| {
                                if *num_matches != 0 {
                                    module.include[idx].tag.clone()
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                })
            })
            .collect();

        Ok(Self {
            name: module_name,
            paths: paths?,
        })
    }
}

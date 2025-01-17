use crate::{
    core::{
        config::{Config, Route},
        fs::{
            reader::{FileContent, FileHolder, Reader},
            Data,
        },
    },
    error::Error,
};

pub struct PreRoutes;

impl PreRoutes {
    pub fn fix(
        config: &mut Config,
    ) -> Result<(), Error> {
        let data =  Reader::new(config.clone().dir.into()).read_other()?;
        for file in data {
            let name = file.path.file_name().unwrap().to_str().unwrap();
            match file.ext.as_str() {
                "html" | "md" => continue,
                _ => config.routes.push(Route {
                    to: name.to_string(),
                    file_name: name.to_string(),
                }),
            }
        }
        Ok(())
    }
}

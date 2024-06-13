use std::path::Path;
use std::io::Write;
use serde_json as json;

use domain::light::Light;
use local_registry::{
    Registry,
    Error,
    Result,
};

const DUMPS: &str = "dumps";
const DEFULATS: &str = "defaults";

struct JSONRegistry {
    location: Path,
}

impl JSONRegistry {
    fn ensure_path<P: AsRef<Path>>(self: &Self, path: P) -> Result<()> {
        if self.location.exists() {
            Ok(())
        } else {
            match std::fs::create_dir_all(path) {
                Err(err) => return Error::internal(self.name(), Box::new(err)),
                Ok(_) => Ok(()),
            }
        }
    }

    fn ensure_paths(self: &Self) -> Result<()> {
        self.ensure_path(&self.location)?;
        self.ensure_path(self.location.join(DUMPS))?;
        self.ensure_path(self.location.join(DEFULATS))
    }

    fn check_light(self: &Self, light: &Light) -> Result<()> {
        if "" == light.name {
            Error::unnamed(self.name())
        } else {
            Ok(())
        }
    }

    fn dump_to_file(self: &Self, subdir: &str, light: &Light) -> Result<()> {
        self.check_light(light)?;
        self.ensure_paths()?;

        let path = self.location
            .join(subdir)
            .join(format!("{}.json", light.name));

        match std::fs::File::create(path) {
            Err(err) => Error::internal(self.name(), Box::new(err)),
            Ok(file) => {
                let mut writer = std::io::BufWriter::new(file);

                if let Err(err) = json::to_writer(&mut writer, &light) {
                    Error::internal(self.name(), Box::new(err))
                } else if let Err(err) = writer.flush() {
                    Error::internal(self.name(), Box::new(err))
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Registry for JSONRegistry {
    fn name(self: &Self) -> &str {
        "json"
    }

    fn list_defaults(self: &Self) -> Result<Vec<Light>> {
        todo!()
    }

    fn list_dumps(self: &Self) -> Result<Vec<Light>> {
        todo!()
    }

    fn load_default(self: &Self, name: &str) -> Result<Light> {
        todo!()
    }

    fn load_dump(self: &Self, name: &str) -> Result<Light> {
        todo!()
    }

    fn dump(self: &mut Self, light: &Light) -> Result<()> {
        self.dump_to_file(DUMPS, light)
    }

    fn default(self: &mut Self, light: &Light) -> Result<()> {
        self.dump_to_file(DEFULATS, light)
    }

    fn remove_dump(self: &mut Self, name: &str) -> Result<()> {
        todo!()
    }

    fn remove_default(self: &mut Self, name: &str) -> Result<()> {
        todo!()
    }

    fn rename_dump(self: &mut Self, old: &str, new: &str) -> Result<()> {
        todo!()
    }

    fn rename_default(self: &mut Self, old: &str, new: &str) -> Result<()> {
        todo!()
    }
}

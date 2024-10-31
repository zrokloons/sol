use crate::build::b_struct::BuildResult;
use crate::build::parameters::Parameters;
use crate::config::Config;
use crate::enums::output::Output;
use crate::util::easy::send_receive;
use crate::util::helpers;
use anyhow::Result as AnyhowResult;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
use log;
use std::fs;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

struct Target {
    file_path: PathBuf,
    dir_path: PathBuf,
}

impl Target {
    fn new(build_id: String, base_path: String) -> Target {
        let path = format!("{base_path}/{build_id}");
        let file = format!("{build_id}.json");
        log::debug!("file_path: {path}/{file}");
        log::debug!("dir_path: {path}");
        Target {
            file_path: Path::new(&format!("{path}/{file}")).to_owned(),
            dir_path: Path::new(&path).to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Build {
    pub result: Option<BuildResult>,
    pub config: Config,
    pub parameters: Parameters,
}

impl Build {
    pub fn new(config: Config) -> AnyhowResult<Build> {
        Ok(Self {
            result: None,
            config,
            parameters: Parameters {
                build_id: String::new(),
                force: false,
            },
        })
    }

    pub fn build_id(&mut self, build_id: String) -> AnyhowResult<&mut Self> {
        self.parameters.build_id = build_id;
        Ok(self)
    }

    pub fn force(&mut self, force: bool) -> AnyhowResult<&mut Self> {
        self.parameters.force = force;
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");
        let target = Target::new(self.parameters.build_id.clone(), self.config.cache.clone());

        if self.parameters.force {
            self.remove_existing(&target.dir_path)?;
        }

        let build_result: BuildResult = match target.file_path.exists() {
            true => self.get_from_cache(&target)?,
            false => self.request(&target)?,
        };

        self.result = Some(build_result);
        Ok(self)
    }

    fn get_from_cache(&self, target: &Target) -> AnyhowResult<BuildResult> {
        log::debug!("Get from cache");
        let fh = std::fs::File::open(&target.file_path)?;
        Ok(serde_json::from_reader(fh)?)
    }

    fn request(&self, target: &Target) -> AnyhowResult<BuildResult> {
        log::debug!("New request");
        let mut data: Vec<u8> = Vec::new();
        let url = format!(
            "https://{}/api/tenant/{}/build/{}",
            self.config.host, self.config.tenant, self.parameters.build_id,
        );

        send_receive(&mut data, &url);
        {
            create_dir(&target.dir_path)?;
            let mut file = File::create(&target.file_path)?;
            file.write_all(data.as_slice())?;
        }
        let tmp: BuildResult = serde_json::from_slice(&data)?;
        Ok(tmp)
    }

    fn remove_existing(&self, dir_path: &Path) -> AnyhowResult<()> {
        log::debug!("Remove build directory: {}", dir_path.to_str().unwrap());
        if dir_path.exists() {
            helpers::remove_dir_files(dir_path)?;
            fs::remove_dir(dir_path)?;
        }
        Ok(())
    }

    pub fn get_string(&self, field: &str) -> AnyhowResult<String> {
        let result = self.result.as_ref().unwrap();
        match field {
            "_id" => Ok(result.uuid.clone()),
            "job_name" => Ok(result.job_name.clone()),
            "start_time" => Ok(result.start_time.clone()),
            "end_time" => Ok(result.end_time.clone()),
            "log_url" => Ok(result.log_url.clone()),
            "nodeset" => Ok(result.nodeset.clone()),
            "pipeline" => Ok(result.pipeline.clone()),
            "event_id" => Ok(result.event_id.clone()),
            "event_timestamp" => Ok(result.event_timestamp.clone()),
            _ => Err(anyhow::anyhow!("Field: {field} not yet supported")),
        }
    }

    pub fn show(&mut self) -> AnyhowResult<()> {
        match self.config.output {
            Output::JSON => self._show_json()?,
            Output::USER => self._show_user()?,
        }
        Ok(())
    }

    fn _show_json(&mut self) -> AnyhowResult<()> {
        println!("{}", serde_json::to_string(&self.result.as_ref().unwrap())?);
        Ok(())
    }

    fn _show_user(&mut self) -> AnyhowResult<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec!["Result", "Pipeline", "URL"]);

        let url = format!(
            "https://{}/t/{}/buildset/{}",
            self.config.host,
            self.config.tenant,
            self.result.as_ref().unwrap().buildset.uuid
        );

        table.add_row(vec![
            Cell::new(self.result.as_ref().unwrap().result.as_ref().unwrap()),
            Cell::new(self.result.as_ref().unwrap().pipeline.clone()),
            Cell::new(url),
        ]);
        Ok(println!("{table}"))
    }
}

use crate::builds::builds_struct::{BuildsResult, Target};
use crate::builds::parameters::Parameters;
use crate::config::Config;
use crate::enums::output::Output;
use crate::util::easy::send_receive;
use anyhow::Result as AnyhowResult;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
use log;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Command {
    NotSet,
    Uuid,
    JobName,
    Change,
}

#[derive(Debug)]
pub struct Builds {
    pub result: Option<Vec<BuildsResult>>,
    command: Command,
    pub target: Option<Target>,
    pub config: Config,
    pub parameters: Parameters,
}

impl Builds {
    pub fn new(config: Config) -> AnyhowResult<Builds> {
        Ok(Self {
            result: None,
            command: Command::NotSet,
            target: None,
            config,
            parameters: Parameters {
                job_name: None,
                change: None,
                patchset: None,
                uuid: None,
                force: false,
                verbose: false,
            },
        })
    }

    pub fn job_name(&mut self, job_name: Option<String>) -> AnyhowResult<&mut Self> {
        if job_name.is_some() {
            self.command = Command::JobName;
        }
        self.parameters.job_name = job_name;
        Ok(self)
    }

    pub fn change(&mut self, change: Option<String>) -> AnyhowResult<&mut Self> {
        if change.is_some() {
            self.command = Command::Change;
        }
        self.parameters.change = change;
        Ok(self)
    }

    pub fn uuid(&mut self, uuid: Option<String>) -> AnyhowResult<&mut Self> {
        if uuid.is_some() {
            self.command = Command::Uuid;
        }
        self.parameters.uuid = uuid;
        Ok(self)
    }

    pub fn patchset(&mut self, patchset: Option<String>) -> AnyhowResult<&mut Self> {
        self.parameters.patchset = patchset;
        Ok(self)
    }

    pub fn force(&mut self, force: bool) -> AnyhowResult<&mut Self> {
        self.parameters.force = force;
        Ok(self)
    }

    pub fn verbose(&mut self, verbose: bool) -> AnyhowResult<&mut Self> {
        self.parameters.verbose = verbose;
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");

        let result = match &self.command {
            Command::Uuid => self.runner_uuid()?,
            Command::Change => self.runner_change()?,
            Command::JobName => self.runner_job_name()?,
            _ => panic!("Not implemented"),
        };

        self.result = Some(result);
        Ok(self)
    }

    fn runner_uuid(&mut self) -> AnyhowResult<Vec<BuildsResult>> {
        self.target = Some(Target::new(
            self.parameters.uuid.clone().unwrap(),
            self.config.cache.clone(),
        ));

        let mut data: Vec<u8> = Vec::new();
        let result: Vec<BuildsResult> = match &self.target.as_ref().unwrap().uuid.exists() {
            true => {
                if self.parameters.force {
                    if let Some(target) = &self.target {
                        target.delete()?;
                    };
                    self.request(&mut data)?
                } else {
                    self.cache()?
                }
            }
            false => self.request(&mut data)?,
        };

        self.update_cache(&data, self.target.as_ref().unwrap().uuid.clone())?;
        Ok(result)
    }

    fn update_cache(&mut self, data: &Vec<u8>, target: std::path::PathBuf) -> AnyhowResult<()> {
        if !data.is_empty() {
            log::debug!("Update cache: {target:#?}");
            std::fs::create_dir(&self.target.as_ref().unwrap().dir)?;
            let mut file = std::fs::File::create(target)?;
            file.write_all(data.as_slice())?;
        }
        Ok(())
    }

    fn runner_change(&mut self) -> AnyhowResult<Vec<BuildsResult>> {
        let mut data: Vec<u8> = Vec::new();
        self.request(&mut data)
    }

    fn runner_job_name(&mut self) -> AnyhowResult<Vec<BuildsResult>> {
        let mut data: Vec<u8> = Vec::new();
        self.request(&mut data)
    }

    fn cache(&self) -> AnyhowResult<Vec<BuildsResult>> {
        log::debug!("Get from cache");
        let fh = std::fs::File::open(&self.target.as_ref().unwrap().uuid)?;
        Ok(serde_json::from_reader(fh)?)
    }

    fn request(&self, data: &mut Vec<u8>) -> AnyhowResult<Vec<BuildsResult>> {
        log::debug!("New request");
        let mut url = format!(
            "https://{}/api/tenant/{}/builds",
            self.config.host, self.config.tenant
        );

        if let Some(job_name) = &self.parameters.job_name {
            url.push_str(&format!("?limit={}", self.config.limit));
            url.push_str(&format!("&job_name={job_name}"));
        }

        if let Some(change) = &self.parameters.change {
            url.push_str(&format!("?change={change}"));
        }

        if let Some(patchset) = &self.parameters.patchset {
            url.push_str(&format!("&patchset={patchset}"));
        }

        if let Some(uuid) = &self.parameters.uuid {
            url.push_str(&format!("?uuid={uuid}"));
        }

        send_receive(data, &url);
        let _debug: serde_json::Value = serde_json::from_slice(data)?;
        log::debug!("{_debug:#?}");

        let tmp: Vec<BuildsResult> = serde_json::from_slice(data)?;
        Ok(tmp)
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
        match self.parameters.verbose {
            true => self._show_user_verbose()?,
            false => self._show_user_default()?,
        };
        Ok(())
    }

    fn _show_user_default(&mut self) -> AnyhowResult<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec!["Result", "Change/Patchset", "URL", "Job"]);

        for e in self.result.as_ref().unwrap().iter() {
            let url = format!(
                "https://{}/t/{}/build/{}",
                self.config.host, self.config.tenant, e.uuid,
            );

            let change_patchset: String = format!("{}/{}", e._ref.change, e._ref.patchset);

            let result = match e.result.as_ref() {
                Some(value) => value,
                None => "N/A",
            };

            table.add_row(vec![
                Cell::new(result),
                Cell::new(change_patchset),
                Cell::new(url),
                Cell::new(e.job_name.clone()),
            ]);
        }
        println!("{table}");
        Ok(())
    }

    fn _show_user_verbose(&mut self) -> AnyhowResult<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                "Result",
                "Duration",
                "Project",
                "Branch",
                "Change/Patchset",
                "URL",
                "Job",
            ]);

        for e in self.result.as_ref().unwrap().iter() {
            let url = format!(
                "https://{}/t/{}/build/{}",
                self.config.host, self.config.tenant, e.uuid,
            );

            let project: String = e._ref.project.to_string();
            let branch: String = e._ref.branch.to_string();
            let change_patchset: String = format!("{}/{}", e._ref.change, e._ref.patchset);

            let result = match e.result.as_ref() {
                Some(value) => value,
                None => "N/A",
            };

            let duration = match e.duration.as_ref() {
                Some(value) => &value.to_string(),
                None => "Not started",
            };

            table.add_row(vec![
                Cell::new(result),
                Cell::new(duration),
                Cell::new(project),
                Cell::new(branch),
                Cell::new(change_patchset),
                Cell::new(url),
                Cell::new(e.job_name.clone()),
            ]);
        }
        println!("{table}");
        Ok(())
    }
}

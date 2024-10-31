use crate::buildsets::bs_struct::BuildSetsResult;
use crate::buildsets::parameters::Parameters;
use crate::config::Config;
use crate::enums::bsresult::BSResults;
use crate::enums::output::Output;
use crate::util::easy::send_receive;
use anyhow::{bail, Result as AnyhowResult};
use log;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSets {
    pub result: Option<Vec<BuildSetsResult>>,
    pub config: Config,
    pub parameters: Parameters,
}

impl BuildSets {
    pub fn new(config: Config) -> AnyhowResult<BuildSets> {
        Ok(Self {
            result: None,
            config,
            parameters: Parameters {
                project: None,
                result: None,
                change: None,
            },
        })
    }

    pub fn result(&mut self, result: Option<BSResults>) -> AnyhowResult<&mut Self> {
        if let Some(result) = result {
            self.parameters.result = Some(result);
        }
        Ok(self)
    }

    pub fn project(&mut self, project: Option<String>) -> AnyhowResult<&mut Self> {
        if let Some(project) = project {
            self.parameters.project = Some(project);
        }
        Ok(self)
    }

    pub fn change(&mut self, change: Option<Number>) -> AnyhowResult<&mut Self> {
        if let Some(change) = change {
            self.parameters.change = Some(change);
        }
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");
        let mut data: Vec<u8> = Vec::new();
        let url = format!(
            "https://{}/api/tenant/{}/buildsets?limit={}",
            self.config.host, self.config.tenant, self.config.limit
        );

        send_receive(&mut data, &url);

        let output: Vec<BuildSetsResult> = serde_json::from_slice(&data)?;
        self.result = Some(output);
        Ok(self)
    }

    pub fn filter(&mut self) -> AnyhowResult<&mut Self> {
        let mut tmp: Vec<BuildSetsResult> = vec![];

        for buildset in self.result.take().unwrap() {
            // result filter
            if let Some(result) = &self.parameters.result {
                if let Some(ref _result) = buildset.result {
                    log::debug!("result filter: {result}, seen: {_result}");
                    if _result != result.as_str() {
                        continue;
                    }
                } else if let Some(result) = &self.parameters.result {
                    log::debug!("result filter: {result}, seen {:#?}", buildset.result);
                    if result != &BSResults::None {
                        continue;
                    }
                }
            }

            // Unsupported
            if buildset.refs.len() > 1 {
                bail!("Unhandled length of ref list > 1");
            }

            // project filter
            if let Some(project) = &self.parameters.project {
                log::debug!(
                    "project filter: {project}, seen: {}",
                    buildset.refs[0].project
                );
                if buildset.refs[0].project != *project.to_string() {
                    continue;
                }
            }

            if let Some(change) = &self.parameters.change {
                log::debug!("change filter: {change}, seen: {}", buildset.refs[0].change);
                if buildset.refs[0].change != serde_json::to_value(change)? {
                    continue;
                }
            }

            log::debug!("Adding {buildset:#?}");
            tmp.push(buildset);
        }

        self.result = Some(tmp);
        Ok(self)
    }

    pub fn show(&mut self) -> AnyhowResult<()> {
        match self.config.output {
            Output::JSON => self._show_json()?,
            Output::USER => self._show_user()?,
        }
        Ok(())
    }

    fn _show_json(&mut self) -> AnyhowResult<()> {
        Ok(println!(
            "{}",
            serde_json::to_string(&self.result.as_ref().unwrap()).unwrap()
        ))
    }

    fn _show_user(&mut self) -> AnyhowResult<()> {
        Ok(println!("{:#?}", self.result))
    }
}

use crate::buildsets::bs_struct::BuildSetsResult;
use crate::buildsets::parameters::Parameters;
use crate::config::Config;
use crate::enums::bsresult::BSResults;
use crate::enums::output::Output;
use crate::util::easy::send_receive;
use anyhow::Result as AnyhowResult;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
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
                uuid: None,
            },
        })
    }

    pub fn result(&mut self, result: Option<BSResults>) -> AnyhowResult<&mut Self> {
        self.parameters.result = result;
        Ok(self)
    }

    pub fn project(&mut self, project: Option<String>) -> AnyhowResult<&mut Self> {
        self.parameters.project = project;
        Ok(self)
    }

    pub fn change(&mut self, change: Option<Number>) -> AnyhowResult<&mut Self> {
        self.parameters.change = change;
        Ok(self)
    }

    pub fn uuid(&mut self, uuid: Option<String>) -> AnyhowResult<&mut Self> {
        self.parameters.uuid = uuid;
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");
        let mut data: Vec<u8> = Vec::new();
        let mut url = format!(
            "https://{}/api/tenant/{}/buildsets?limit={}",
            self.config.host, self.config.tenant, self.config.limit
        );

        if let Some(result) = self.parameters.result.as_ref() {
            url.push_str(&format!("&result={result}"));
        }

        if let Some(uuid) = self.parameters.uuid.as_ref() {
            url.push_str(&format!("&uuid={uuid}"));
        }

        if let Some(project) = self.parameters.project.as_ref() {
            url.push_str(&format!("&project={project}"));
        }

        if let Some(change) = self.parameters.change.as_ref() {
            url.push_str(&format!("&change={change}"));
        }

        send_receive(&mut data, &url);

        let output: Vec<BuildSetsResult> = serde_json::from_slice(&data)?;
        self.result = Some(output);
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
        match self.result.as_ref() {
            None => Ok(println!("Nothing found")),
            Some(result) => {
                let mut table = Table::new();
                table
                    .load_preset(UTF8_BORDERS_ONLY)
                    .set_content_arrangement(ContentArrangement::DynamicFullWidth)
                    .set_header(vec![
                        "Result",
                        "Pipeline",
                        "Project",
                        "Branch",
                        "Change/Patchset",
                        "URL",
                    ]);
                for res in result.iter() {
                    let change_patchset = format!(
                        "{}/{}",
                        res.refs[0].change,
                        res.refs[0].patchset.as_ref().unwrap_or(&"?".to_string())
                    );

                    let url = format!(
                        "https://{}/t/{}/buildset/{}",
                        self.config.host, self.config.tenant, res.uuid
                    );
                    table.add_row(vec![
                        Cell::new(res.result.as_ref().unwrap_or(&"N/A".to_string())),
                        Cell::new(res.pipeline.clone()),
                        Cell::new(res.refs[0].project.clone()),
                        Cell::new(res.refs[0].branch.clone()),
                        Cell::new(change_patchset),
                        Cell::new(url),
                    ]);
                }

                Ok(println!("{table}"))
            }
        }
    }
}

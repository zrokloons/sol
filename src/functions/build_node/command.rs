use crate::build;
use crate::config::Config;
use crate::enums::output::Output;
use crate::functions::build_node::parameters::Parameters;
use crate::util::diffdatetime_now::{self, DiffDateTimeNow};
use crate::util::easy::send_receive;
use anyhow::Result as AnyhowResult;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
use flate2::read::GzDecoder;
use log;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct _Node {
    pub name: String,
    pub ip: String,
    pub label: String,
    pub age: DiffDateTimeNow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildNode {
    pub result: Vec<_Node>,
    pub config: Config,
    pub parameters: Parameters,
}

impl BuildNode {
    pub fn new(config: Config) -> AnyhowResult<BuildNode> {
        Ok(Self {
            result: vec![],
            parameters: Parameters {
                build_id: String::new(),
            },
            config,
        })
    }

    pub fn build_id(&mut self, build_id: String) -> AnyhowResult<&mut Self> {
        self.parameters.build_id = build_id;
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");

        // TODO: Possible to use the Target struct, potential refactor can be done here
        let dest: String =
            self.config.cache.clone() + "/" + &self.parameters.build_id + "/inventory.yaml.gz";

        // Get log_url from Build
        let mut build = build::command::Build::new(self.config.clone())?;
        build.build_id(self.parameters.build_id.clone())?;
        build.runner()?;

        let log_url = build.get_string("log_url")?;
        let end_time = build.get_string("end_time")?;
        let age = diffdatetime_now::DiffDateTimeNow::new(format!("{}+00:00", end_time.clone()));

        // Create the url to the inventory using fetched log_url
        let url = format!("{}zuul-info/inventory.yaml", log_url);

        // Download and write to file
        let mut data: Vec<u8> = vec![];
        send_receive(&mut data, &url);
        {
            let mut file = File::create(&dest)?;
            file.write_all(data.as_slice())?;
        }

        // Decompress file
        let in_filename = dest;
        let in_fh = std::fs::File::open(in_filename)?;
        let mut gz = GzDecoder::new(in_fh);
        let mut store = String::new();
        gz.read_to_string(&mut store)?;

        self.parse_n_populate(&mut store, age)?;
        Ok(self)
    }

    /*
     * This function parse the inventory.yaml file, create a Node object and store
     * on the result.
     *
     * TODO: Refactor this function
     */
    fn parse_n_populate(
        &mut self,
        store: &mut str,
        age: DiffDateTimeNow,
    ) -> AnyhowResult<&mut Self> {
        let data: serde_yaml::Value = serde_yaml::from_str(store)?;
        let hosts = data["all"]["hosts"].to_owned();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(hosts)?;
        let mut collected: Vec<String> = vec![];

        for (host, value) in map.iter() {
            let m2: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(value.to_owned())?;
            for (attr, v2) in m2.iter() {
                if attr == "ansible_host" {
                    collected.push(v2.as_str().unwrap().to_string());
                }
                if attr == "nodepool" {
                    let m3: HashMap<String, serde_yaml::Value> =
                        serde_yaml::from_value(v2.to_owned())?;
                    for (attr2, v3) in m3.iter() {
                        if attr2 == "label" {
                            collected.push(v3.as_str().unwrap().to_string());
                        }
                    }
                }
                if collected.len() == 2 {
                    self.result.push(_Node {
                        name: host.to_string(),
                        ip: collected[0].clone(),
                        label: collected[1].clone(),
                        age: age.clone(),
                    });
                    collected = vec![];
                }
            }
        }
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
        for node in self.result.iter() {
            println!("{}", serde_json::to_string(node).unwrap());
        }
        Ok(())
    }

    fn _show_user(&mut self) -> AnyhowResult<()> {
        let mut table = Table::new();
        table
            .load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec!["Node", "IP addr", "Label", "Age"]);

        for node in self.result.iter() {
            table.add_row(vec![
                Cell::new(node.name.clone()),
                Cell::new(node.ip.clone()),
                Cell::new(node.label.clone()),
                Cell::new(node.age.clone()),
            ]);
        }

        Ok(println!("{table}"))
    }
}

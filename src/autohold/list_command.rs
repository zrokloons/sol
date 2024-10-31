use crate::autohold::list_parameters::Parameters;
use crate::autohold::list_struct::AutoHoldResult;
use crate::config::Config;
use crate::enums::output::Output;
use crate::util::easy::send_receive;
use anyhow::Result as AnyhowResult;
use chrono::prelude::DateTime;
use chrono::FixedOffset;
use chrono::Utc;
use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::*;
use log;
use regex::Regex;
use std::time::{Duration, UNIX_EPOCH};

#[derive(Debug)]
pub struct ListAutoHold {
    pub result: Option<Vec<AutoHoldResult>>,
    pub config: Config,
    pub parameters: Parameters,
}

impl ListAutoHold {
    pub fn new(config: Config) -> AnyhowResult<ListAutoHold> {
        Ok(Self {
            result: None,
            config,
            parameters: Parameters {
                user: None,
                snapped: true,
            },
        })
    }

    pub fn user(&mut self, user: Option<String>) -> AnyhowResult<&mut Self> {
        if let Some(user) = user {
            self.parameters.user = Some(user)
        }
        Ok(self)
    }

    pub fn snapped(&mut self, snapped: bool) -> AnyhowResult<&mut Self> {
        self.parameters.snapped = snapped;
        Ok(self)
    }

    pub fn runner(&mut self) -> AnyhowResult<&mut Self> {
        log::debug!("{self:#?}");
        let mut data: Vec<u8> = Vec::new();
        let url = format!(
            "https://{}/api/tenant/{}/autohold",
            self.config.host, self.config.tenant,
        );

        send_receive(&mut data, &url);

        let output: Vec<AutoHoldResult> = serde_json::from_slice(&data)?;
        self.result = Some(output);
        Ok(self)
    }

    pub fn filter(&mut self) -> AnyhowResult<&mut Self> {
        let mut tmp: Vec<AutoHoldResult> = vec![];

        for autohold in self.result.take().unwrap() {
            // User filter
            if let Some(user) = &self.parameters.user {
                let pattern = format!("{user}@{}", &self.config.filters.autohold_user);
                log::debug!("User filter: {pattern}");

                if !autohold.reason.contains(&pattern) {
                    continue;
                }
            }

            // Snapped filter
            if autohold.current_count == 0 && self.parameters.snapped {
                continue;
            }

            tmp.push(autohold);
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
        let mut table = Table::new();
        table
            .load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                "Snapped", "Owner", "Build", "Expire", "Project", "Job",
            ]);

        for elem in self.result.as_ref().unwrap() {
            let snapped = match elem.current_count == elem.max_count {
                true => format!("😁 [{:#?}/{:#?}]", elem.current_count, elem.max_count),
                false => format!("🫥 [{:#?}/{:#?}]", elem.current_count, elem.max_count),
            };

            let mut builds: Vec<&str> = vec![];
            match elem.nodes.len() {
                0 => (),
                _ => {
                    for node in elem.nodes.iter() {
                        builds.push(&node.build);
                    }
                }
            };

            let regexp = Regex::new("by (.*)@")?;
            let cap = regexp.captures(&elem.reason).unwrap();

            let expire = match elem.expired {
                Some(value) => {
                    // Creates a new SystemTime from the specified number of whole seconds
                    let d = UNIX_EPOCH
                        + Duration::from_secs(value as u64 + elem.node_expiration as u64);
                    // Create DateTime from SystemTime
                    let datetime = DateTime::<Utc>::from(d);
                    // Formats the combined date and time with the specified format string.
                    let g = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

                    let now = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
                    match now > datetime {
                        true => format!("🫥 {}", g),
                        false => format!("😀 {}", g),
                    }
                }
                None => "N/A".to_string(),
            };

            table.add_row(vec![
                Cell::new(snapped),
                Cell::new(&cap[1]),
                Cell::new(builds.join("\n")),
                Cell::new(expire),
                Cell::new(elem.project.clone()),
                Cell::new(elem.job.clone()),
            ]);
        }
        Ok(println!("{table}"))
    }
}

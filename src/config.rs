use crate::cli_struct::Cli;
use crate::enums::output::Output;
use anyhow::Result as AnyhowResult;
use lazy_static::lazy_static;
use log;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

lazy_static! {
    static ref _DEFAULT_SOL_CONFIG_PATH2: String = String::from(".config/sol/");
    static ref _DEFAULT_SOL_CONFIG_NAME: String = String::from("config.yml");
    static ref SOL_CONFIG: String = {
        if let Ok(val) = env::var("SOL_CONFIG_PATH") {
            val
        } else {
            let mut tmp = String::new();
            tmp.push_str(&env::var("HOME").unwrap());
            tmp.push('/');
            tmp.push_str(&_DEFAULT_SOL_CONFIG_PATH2);
            tmp.push_str(&_DEFAULT_SOL_CONFIG_NAME);
            tmp
        }
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    pub autohold_user: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    // Default Tenant
    pub tenant: String,

    // Cache path
    pub cache: String,

    // Zuul host
    pub host: String,

    // Output format
    pub output: Output,

    // API limit
    pub limit: usize,

    // Filters
    pub filters: Filter,
}

impl Default for Config {
    fn default() -> Self {
        let mut cache: String = String::new();
        cache.push_str(&env::var("HOME").unwrap());
        cache.push_str("/.sol/");

        Self {
            tenant: String::from("TENANT"),
            cache,
            host: String::from("ZUUL_HOST"),
            output: Output::USER,
            limit: 10,
            filters: Filter {
                autohold_user: "PATTERN".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load(cli: &Cli) -> AnyhowResult<Config> {
        if !Path::new(&SOL_CONFIG.as_str()).exists() {
            Config::init()?;
        }

        let contents = fs::read_to_string(SOL_CONFIG.as_str())?;
        log::debug!("Load config from: {:#?}", SOL_CONFIG.as_str());
        let mut config: Config = serde_yaml::from_str(&contents)?;
        log::debug!("Loaded config: {:#?}", config);

        // Override configuration with options
        if let Some(tenant) = cli.tenant.as_ref() {
            config.tenant = tenant.clone();
        }
        config.limit = cli.limit;
        config.output = cli.output;
        log::debug!("Config after overrides: {:#?}", config);
        Ok(config)
    }

    fn init() -> AnyhowResult<()> {
        println!("Initializing new configuration file...");

        // Tets if we have directory path
        let path: String = format!(
            "{}/{}",
            env::var("HOME")?,
            _DEFAULT_SOL_CONFIG_PATH2.as_str()
        );
        if !Path::new(&path).exists() {
            print!("Create path: {} ? [y/n]", path);
            io::stdout().flush()?;
            let mut answer = String::new();
            io::stdin().read_line(&mut answer)?;

            match answer.trim() {
                "y" | "Y" => (),
                _ => {
                    println!("Aborted");
                    std::process::exit(1);
                }
            };

            fs::create_dir_all(path)?;
        }

        // Create a default configuration file
        let config = Config::default();
        let data = serde_yaml::to_string(&config).unwrap();
        fs::write(SOL_CONFIG.as_str(), data).unwrap();

        println!("Sol configuration created! {}", SOL_CONFIG.as_str());
        std::process::exit(0);
    }
}

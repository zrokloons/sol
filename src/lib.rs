extern crate curl;
extern crate log;
extern crate serde_json;

pub mod util {
    pub mod diffdatetime_now;
    pub mod easy;
    pub mod helpers;
}

pub mod buildsets {
    pub mod bs_struct;
    pub mod cli;
    pub mod command;
    pub mod parameters;
}

pub mod autohold {
    pub mod cli;
    pub mod list_command;
    pub mod list_parameters;
    pub mod list_struct;
}

pub mod builds {
    pub mod builds_struct;
    pub mod cli;
    pub mod command;
    pub mod parameters;
}

pub mod enums {
    pub mod bsresult;
    pub mod output;
}

pub mod functions {
    pub mod cli;
    pub mod build_node {
        pub mod cli;
        pub mod command;
        pub mod parameters;
    }
}

pub mod cli_struct;
pub mod config;

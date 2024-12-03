use curl::easy::Easy;
#[allow(unused_imports)]
use log;
use serde_derive::Deserialize;
use std::process;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(flatten)]
    pub other: serde_json::Value,
}

pub fn send_receive(data: &mut Vec<u8>, url: &str) {
    log::debug!("url: {}", url);
    let mut easy = Easy::new();

    easy.url(url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|bits| {
                data.extend_from_slice(bits);
                Ok(bits.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    match easy.response_code() {
        Ok(code) => match code {
            200 => log::debug!("Response code: {code}"),
            _ => {
                println!("Response code: {}", code);
                let message: String = serde_json::from_slice(data).unwrap();
                println!("{:#?}", message);
                process::exit(1);
            }
        },
        Err(err) => panic!("Error: {}", err),
    }
}

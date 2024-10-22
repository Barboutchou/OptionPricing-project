#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

mod boxtoolsapi;

extern crate reqwest;
extern crate tokio ;

use reqwest::header::{HeaderMap, HeaderValue};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let symbol = "ADFI" ;
    match boxtoolsapi::get_corporate_actions(symbol).await {
        Ok(data) => {
            println!("Received data: {}", data);
        }
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
        }
    }
    Ok(())
}


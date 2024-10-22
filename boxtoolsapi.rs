use std::error::Error;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Url;

async fn get_api( url : Url  ) -> Result<String, Box<dyn Error>> {

    let api_key = "PKXCFETH7CJ66CG4W0XP";
    let api_private_key = "ebfIVLtdHWLaihD10WK7ub7gBioWnLPlpF1Bmbs2";

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("application/json"));
    headers.insert("APCA-API-KEY-ID", HeaderValue::from_static(api_key));
    headers.insert("APCA-API-SECRET-KEY", HeaderValue::from_static(api_private_key));

    // Send the GET request
    let response = client
        .get(url)          // Target URL
        .headers(headers)  // Adding headers
        .send()            // Sending the request
        .await?;

    // Check if the response was successful
    if response.status().is_success() {
        let body = response.text().await?; // Await the response body
        println!("{}", body); // Print the response body
        Ok(body) // Return the body as a String
    } else {
        Err(Box::from(format!("Request failed with status: {}", response.status())))
    }
}

pub async fn get_assets( symbol : &str ) -> Result<String, Box<dyn Error>> {
    let base_url = format!( "https://paper-api.alpaca.markets/v2/assets/{}" , symbol  ) ;
    let url = Url::parse(base_url.as_str()).expect("Failed to parse URL");
    return get_api( url ).await ;
}

pub async fn get_corporate_actions( symbol : &str ) -> Result<String, Box<dyn Error>> {
    let base_url = "https://data.alpaca.markets/v1/corporate-actions" ;
    let mut url = Url::parse(base_url)?.join(symbol)?;
    url.query_pairs_mut()
        .append_pair("symbols", symbol )
        .append_pair( "start" , "2023-01-01")
        .append_pair("end", "2024-10-21") ;
    println!("{:?}", url);
    return get_api( url ).await ;
}
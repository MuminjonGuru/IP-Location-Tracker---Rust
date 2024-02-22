use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct IpLocation {
    ip: String,
    country_name: String,
    city: String,
    // Add more fields as needed
}

async fn get_ip_location(client: &Client, api_key: &str, ip: &str) -> Result<IpLocation, Error> {
    let url = format!("http://api.ipstack.com/{}?access_key={}", ip, api_key);
    // println!("{}", url); // check if the url is valid
    let response = client.get(&url).send().await?.json::<IpLocation>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_key = "YOUR_API_KEY_GOES_HERE";
    let ip = "134.201.250.155"; // Example IP, could also be passed as an argument or read from env

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let location = get_ip_location(&client, &api_key, ip).await?;
    println!(
        "IP: {}, Country: {}, City: {}",
        location.ip, location.country_name, location.city
    );
    Ok(())
}

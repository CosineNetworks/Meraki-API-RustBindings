use std::io::{self, Write};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tokio::runtime::Builder;

async fn get_switch_ports() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", "1ba26cd3769e4436b10e211a1d6ead5ef8025330".parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/{{serial}}/switch/ports")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn get_organizations() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Cisco-Meraki-API-Key", "1ba26cd3769e4436b10e211a1d6ead5ef8025330".parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/organizations")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn get_switch_status() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", "1ba26cd3769e4436b10e211a1d6ead5ef8025330".parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/switch/ports/statuses")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn handle_request(request: &str) -> Result<(), Box<dyn std::error::Error>> {
    match request {
        "switch_ports" => get_switch_ports().await?,
        "organizations" => get_organizations().await?,
        "get_switch_status" => get_switch_status().await?,
        _ => println!("Invalid request."),
    }
    Ok(())
}

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    
    loop {
        print!("Enter request: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let request = input.trim();

        if request == "exit" {
            break;
        }

        rt.block_on(async {
            if let Err(e) = handle_request(request).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

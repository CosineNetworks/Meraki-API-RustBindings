use std::io::{self, Write};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tokio::runtime::Builder;

const MERAKI_API_KEY: &str = "YOUR API KEY";

// List the switch ports for a switch 
async fn get_switch_ports() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/{{serial}}/switch/ports")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List organizations
async fn get_organizations() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/organizations")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return The Status For All The Ports Of A Switch
async fn get_switch_status() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/switch/ports/statuses")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return the SSID statuses of an access point
async fn ap_ssid_status() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/wireless/status")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Returns the identity of the current user.
async fn identities() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/administered/identities/me")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return the DHCP subnet information for an appliance
async fn DHCP_subnet_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/appliance/dhcp/subnets")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return the performance score for a single MX
async fn device_performance() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/appliance/performance")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return Current Delegated IPv6 Prefixes On An Appliance
async fn delegated_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/appliance/prefixes/delegated")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return prefixes assigned to all IPv6 enabled VLANs on an appliance.
async fn vlan_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let request = client
        .get("https://api.meraki.com/api/v1/devices/VRT-2207620607704/appliance/prefixes/delegated/vlanAssignments")
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
        "ap_ssid_status" => ap_ssid_status().await?,
        "identities" => identities().await?,
        "DHCP_subnet_info" => DHCP_subnet_info().await?,
        "device_performance" => device_performance().await?,
        "delegated_prefix" => delegated_prefix().await?,
        "vlan_prefix" => vlan_prefix().await?,
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

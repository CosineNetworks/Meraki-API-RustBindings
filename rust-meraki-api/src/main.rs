use std::io::{self, Write};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tokio::runtime::Builder;

const MERAKI_API_KEY: &str = "";
const serial: &str = "";
const organizationId: &str = "";
const networkId: &str = "";

// List the switch ports for a switch 
async fn get_switch_ports() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/switch/ports", serial);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the switchports in an organization by switch
async fn list_org_switchports() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/switch/ports/bySwitch", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return A Switch Port
async fn switch_port_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607762".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/switch/ports/1", serial);

    let request = client
        .get(endpoint)
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

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/switch/ports/statuses", serial);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return the packet counters for all the ports of a switch
async fn switch_packet_count() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/switch/ports/statuses/packets", serial);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Returns The Switch Network Settings
async fn switch_network_settings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/switch/settings", networkId);

    let request = client
        .get(endpoint)
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

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/wireless/status", serial);

    let request = client
        .get(endpoint)
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

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/appliance/dhcp/subnets", serial);

    let request = client
        .get(endpoint)
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

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/appliance/performance", serial);

    let request = client
        .get(endpoint)
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

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/appliance/prefixes/delegated", serial);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return prefixes assigned to all IPv6 enabled VLANs on an appliance.
async fn list_devices() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "VRT-2207620607704".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}/appliance/prefixes/delegated/vlanAssignments", serial);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the devices in an organization
async fn vlan_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Show VPN status for networks in an organization
async fn vpn_status() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/appliance/vpn/statuses", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Show VPN history stat for networks in an organization
async fn vpn_history_stats() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/appliance/vpn/stats", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return The Firewall Rules For An Organizations Site To Site VPN
async fn firewall_rules() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/appliance/vpn/vpnFirewallRules", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List The VLA Ns For An MX Network
async fn list_vlans() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/appliance/vlans", networkId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn handle_request(request: &str) -> Result<(), Box<dyn std::error::Error>> {
    match request {
        "get_switch_ports" => get_switch_ports().await?,
        "list_org_switchports" => list_org_switchports().await?,
        "switch_port_info" => switch_port_info().await?,
        "get_organizations" => get_organizations().await?,
        "get_switch_status" => get_switch_status().await?,
        "switch_packet_count" => switch_packet_count().await?,
        "switch_network_settings" => switch_network_settings().await?,
        "ap_ssid_status" => ap_ssid_status().await?,
        "identities" => identities().await?,
        "DHCP_subnet_info" => DHCP_subnet_info().await?,
        "device_performance" => device_performance().await?,
        "delegated_prefix" => delegated_prefix().await?,
        "vlan_prefix" => vlan_prefix().await?,
        "list_devices" => list_devices().await?,
        "vpn_status" => vpn_status().await?,
        "vpn_history_stats" => vpn_history_stats().await?,
        "firewall_rules" => firewall_rules().await?,
        "list_vlans" => list_vlans().await?,
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

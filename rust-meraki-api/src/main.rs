use std::io::{self, Write};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tokio::runtime::Builder;

const MERAKI_API_KEY: &str = "";
const serial: &str = "";
const organizationId: &str = "";
const networkId: &str = "";
const SSID_Num: &str = "";

// Create A Network
async fn create_network() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "name": "<string>",
        "productTypes": [
            "<string>",
            "<string>"
        ],
        "tags": [
            "<string>",
            "<string>"
        ],
        "timeZone": "<string>",
        "copyFromNetworkId": "<string>",
        "notes": "<string>"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/networks", organizationId);

    let request = client
        .post(endpoint)
        .body(data)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Create A Network
async fn combine_networks() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "name": "<string>",
        "networkIds": [
            "<string>",
            "<string>"
        ],
        "enrollmentString": "<string>"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/networks/combine", organizationId);

    let request = client
        .post(endpoint)
        .body(data)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

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

// Update The Attributes Of A Device
async fn update_device_attributes() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "name": "<string>",
        "tags": [
            "<string>",
            "<string>"
        ],
        "lat": "<float>",
        "lng": "<float>",
        "address": "<string>",
        "notes": "<string>",
        "moveMapMarker": "<boolean>",
        "switchProfileId": "<string>",
        "floorPlanId": "<string>"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/devices/{}", serial);

    let request = client
        .put(endpoint)
        .body(data)
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

// List The VPN Settings For The SSID
async fn list_ssid_vpn_settings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/wireless/ssids/{}/vpn", networkId, SSID_Num);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Update The VPN Settings For The SSID
async fn update_ssid_vpn_settings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "concentrator": {
            "networkId": "<string>",
            "vlanId": "<integer>"
        },
        "splitTunnel": {
            "enabled": "<boolean>",
            "rules": [
                {
                    "destCidr": "<string>",
                    "policy": "<string>",
                    "protocol": "<string>",
                    "destPort": "<string>",
                    "comment": "<string>"
                },
                {
                    "destCidr": "<string>",
                    "policy": "<string>",
                    "protocol": "<string>",
                    "destPort": "<string>",
                    "comment": "<string>"
                }
            ]
        },
        "failover": {
            "requestIp": "<string>",
            "heartbeatInterval": "<integer>",
            "idleTimeout": "<integer>"
        }
    }"#;  

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/wireless/ssids/{}/vpn", networkId, SSID_Num);

    let request = client
        .put(endpoint)
        .body(data)
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

// List the uplink status of every Meraki MX and Z series appliances in the organization
async fn list_uplink_status() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/appliance/uplink/statuses", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return The Wireless Settings For A Network
async fn return_wireless_settings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/wireless/settings", networkId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Update The Wireless Settings For A Network
async fn update_wireless_settings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "meshingEnabled": "<boolean>",
        "ipv6BridgeEnabled": "<boolean>",
        "locationAnalyticsEnabled": "<boolean>",
        "upgradeStrategy": "<string>",
        "ledLightsOn": "<boolean>"
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/wireless/settings", networkId);

    let request = client
        .put(endpoint)
        .body(data)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

async fn handle_request(request: &str) -> Result<(), Box<dyn std::error::Error>> {
    match request {
        "create_network" => create_network().await?,
        "combine_networks" => combine_networks().await?,
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
        "update_device_attributes" => update_device_attributes().await?,
        "vpn_status" => vpn_status().await?,
        "vpn_history_stats" => vpn_history_stats().await?,
        "list_ssid_vpn_settings" => list_ssid_vpn_settings().await?,
        "update_ssid_vpn_settings" => update_ssid_vpn_settings().await?,
        "firewall_rules" => firewall_rules().await?,
        "list_vlans" => list_vlans().await?,
        "list_uplink_status" => list_uplink_status().await?,
        "return_wireless_settings" => return_wireless_settings().await?,
        "update_wireless_settings" => update_wireless_settings().await?,
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

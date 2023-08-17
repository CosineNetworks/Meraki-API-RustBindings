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
async fn createOrganizationNetwork() -> Result<(), Box<dyn std::error::Error>> {
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

// Combine Multiple Networks Into A Single Network
async fn combineOrganizationNetworks() -> Result<(), Box<dyn std::error::Error>> {
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

// Create A New Organization
async fn createOrganization() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    // Change to your settings
    let data = r#"{
        "name": "<string>",
        "management": {
            "details": [
                {
                    "name": "<string>",
                    "value": "<string>"
                },
                {
                    "name": "<string>",
                    "value": "<string>"
                }
            ]
        }
    }"#;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    let endpoint = format!("https://api.meraki.com/api/v1/organizations");

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
async fn getDeviceSwitchPorts() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getOrganizationSwitchPortsBySwitch() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getDeviceSwitchPort() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getOrganizations() -> Result<(), Box<dyn std::error::Error>> {
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

// List the devices in an organization
async fn getOrganizationDevices() -> Result<(), Box<dyn std::error::Error>> {
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

// Return The Status For All The Ports Of A Switch
async fn getDeviceSwitchPortsStatuses() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getDeviceSwitchPortsStatusesPackets() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getNetworkSwitchSettings() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getDeviceWirelessStatus() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getAdministeredIdentitiesMe() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getDeviceApplianceDhcpSubnets() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getDeviceAppliancePerformance() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn getDeviceAppliancePrefixesDelegated() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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

// Return prefixes assigned to all IPv6 enabled VLANs on an appliance
async fn getDeviceAppliancePrefixesDelegatedVlanAssignments() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
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
async fn updateDevice() -> Result<(), Box<dyn std::error::Error>> {
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

// Show VPN status for networks in an organization
async fn getOrganizationApplianceVpnStatuses() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getOrganizationApplianceVpnStats() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getNetworkWirelessSsidVpn() -> Result<(), Box<dyn std::error::Error>> {
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
async fn updateNetworkWirelessSsidVpn() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getOrganizationApplianceVpnVpnFirewallRules() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getNetworkApplianceVlans() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getOrganizationApplianceUplinkStatuses() -> Result<(), Box<dyn std::error::Error>> {
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
async fn getNetworkWirelessSettings() -> Result<(), Box<dyn std::error::Error>> {
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
async fn updateNetworkWirelessSettings() -> Result<(), Box<dyn std::error::Error>> {
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

// Get average channel utilization across all bands for all networks in the organization
async fn getOrganizationWirelessDevicesChannelUtilizationByNetwork() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/wireless/devices/channelUtilization/byNetwork", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return an overview of current device statuses
async fn getOrganizationDevicesStatusesOverview() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/statuses/overview", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List The Status Of Every Meraki Device In The Organization
async fn getOrganizationDevicesStatuses() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/statuses", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the current uplink addresses for devices in an organization
async fn getOrganizationDevicesUplinksAddressesByDevice() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/uplinks/addresses/byDevice", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return the uplink loss and latency for every MX in the organization from at latest 2 minutes ago
async fn getOrganizationDevicesUplinksLossAndLatency() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/uplinksLossAndLatency", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the provisioning statuses information for devices in an organization
async fn getOrganizationDevicesProvisioningStatuses() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/provisioning/statuses", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the power status information for devices in an organization
async fn getOrganizationDevicesPowerModulesStatusesByDevice() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/powerModules/statuses/byDevice", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the availability information for devices in an organization
async fn getOrganizationDevicesAvailabilities() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/devices/availabilities", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// List the most recent Ethernet link speed, duplex, aggregation and power mode and status information for wireless devices
async fn getOrganizationWirelessDevicesEthernetStatuses() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/organizations/{}/wireless/devices/ethernet/statuses", organizationId);

    let request = client
        .get(endpoint)
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}

// Return average wireless latency over time for a network, device, or network client
async fn getNetworkWirelessLatencyHistory() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "".parse()?);
    headers.insert("X-Cisco-Meraki-API-Key", MERAKI_API_KEY.parse()?);

    let endpoint = format!("https://api.meraki.com/api/v1/networks/{}/wireless/latencyHistory", networkId);

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
        "createOrganizationNetwork" => createOrganizationNetwork().await?,
        "combineOrganizationNetworks" => combineOrganizationNetworks().await?,
        "createOrganization" => createOrganization().await?,
        "getDeviceSwitchPorts" => getDeviceSwitchPorts().await?,
        "getOrganizationSwitchPortsBySwitch" => getOrganizationSwitchPortsBySwitch().await?,
        "getDeviceSwitchPort" => getDeviceSwitchPort().await?,
        "getOrganizations" => getOrganizations().await?,
        "getOrganizationDevices" => getOrganizationDevices().await?,
        "getDeviceSwitchPortsStatuses" => getDeviceSwitchPortsStatuses().await?,
        "getDeviceSwitchPortsStatusesPackets" => getDeviceSwitchPortsStatusesPackets().await?,
        "getNetworkSwitchSettings" => getNetworkSwitchSettings().await?,
        "getDeviceWirelessStatus" => getDeviceWirelessStatus().await?,
        "getAdministeredIdentitiesMe" => getAdministeredIdentitiesMe().await?,
        "getDeviceApplianceDhcpSubnets" => getDeviceApplianceDhcpSubnets().await?,
        "getDeviceAppliancePerformance" => getDeviceAppliancePerformance().await?,
        "getDeviceAppliancePrefixesDelegated" => getDeviceAppliancePrefixesDelegated().await?,
        "getDeviceAppliancePrefixesDelegatedVlanAssignments" => getDeviceAppliancePrefixesDelegatedVlanAssignments().await?,
        "updateDevice" => updateDevice().await?,
        "getOrganizationApplianceVpnStatuses" => getOrganizationApplianceVpnStatuses().await?,
        "getOrganizationApplianceVpnStats" => getOrganizationApplianceVpnStats().await?,
        "getNetworkWirelessSsidVpn" => getNetworkWirelessSsidVpn().await?,
        "updateNetworkWirelessSsidVpn" => updateNetworkWirelessSsidVpn().await?,
        "getOrganizationApplianceVpnVpnFirewallRules" => getOrganizationApplianceVpnVpnFirewallRules().await?,
        "getNetworkApplianceVlans" => getNetworkApplianceVlans().await?,
        "getOrganizationApplianceUplinkStatuses" => getOrganizationApplianceUplinkStatuses().await?,
        "getNetworkWirelessSettings" => getNetworkWirelessSettings().await?,
        "updateNetworkWirelessSettings" => updateNetworkWirelessSettings().await?,
        "getOrganizationWirelessDevicesChannelUtilizationByNetwork" => getOrganizationWirelessDevicesChannelUtilizationByNetwork().await?,
        "getOrganizationDevicesStatusesOverview" => getOrganizationDevicesStatusesOverview().await?,
        "getOrganizationDevicesStatuses" => getOrganizationDevicesStatuses().await?,
        "getOrganizationDevicesUplinksAddressesByDevice" => getOrganizationDevicesUplinksAddressesByDevice().await?,
        "getOrganizationDevicesUplinksLossAndLatency" => getOrganizationDevicesUplinksLossAndLatency().await?,
        "getOrganizationDevicesProvisioningStatuses" => getOrganizationDevicesProvisioningStatuses().await?,
        "getOrganizationDevicesPowerModulesStatusesByDevice" => getOrganizationDevicesPowerModulesStatusesByDevice().await?,
        "getOrganizationDevicesAvailabilities" => getOrganizationDevicesAvailabilities().await?,
        "getOrganizationWirelessDevicesEthernetStatuses" => getOrganizationWirelessDevicesEthernetStatuses().await?,
        "getNetworkWirelessLatencyHistory" => getNetworkWirelessLatencyHistory().await?,
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

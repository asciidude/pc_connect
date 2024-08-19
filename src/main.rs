use std::process::exit;

/**
 * Add this program to your startup menu.
 * It will send computer login information through a Discord webhook every time
 * the process starts, stopping the moment it's finished.
 */

use public_ip;
use local_ip_address::{local_ip, local_ipv6};
use discord_webhook2::{message::Message, webhook::DiscordWebhook};
use geolocation;
use whoami::{self, fallible};
use chrono::Utc;
 
 #[tokio::main]
async fn main() {
    println!("Gathering information...");
 
    let webhook_url = "";

    let public_ip_addr = public_ip::addr().await;

    let ip_addr: String = match public_ip_addr {
        Some(ip) => ip.to_string(),
        None => "None".to_string(),
    };

    let ipv4_addr: String = match local_ip() {
        Ok(ipv4) => ipv4.to_string(),
        Err(_) => "None".to_string(),
    };

    let ipv6_addr: String = match local_ipv6() {
        Ok(ipv6) => ipv6.to_string(),
        Err(_) => "None".to_string(),
    };

    println!("Public IP: {}\nIPv4: {}\nIPv6: {}", ip_addr, ipv4_addr, ipv6_addr);

    let webhook = DiscordWebhook::new(webhook_url).unwrap();
    let mut message_content = format!("# :warning: A new login has been detected on your computer\n## :wireless: Network Information\nPublic IP Address: {}\nIPv4 Address: {}\nIPv6 Address: {}", ip_addr, ipv4_addr, ipv6_addr);

    if ip_addr != "None" {
        let locator = geolocation::find(&ip_addr).unwrap();
        message_content += format!(
            "\n## :globe_with_meridians: Geolocation Information\nCoordinates: {}, {}\nCity: {}\nRegion: {}\nCountry: {}\nTimezone: {}",
            locator.latitude,
            locator.longitude,
            locator.city.trim_matches('"'),
            locator.region.trim_matches('"'),
            locator.country.trim_matches('"'),
            locator.timezone.trim_matches('"')
        ).as_str();
    }

    let username = whoami::username();
    let hostname = fallible::hostname();
    let os = whoami::distro();
    let timestamp = Utc::now().to_rfc3339();

    message_content += format!(
        "\n## :computer: Computer Information\nUsername: {}\nHostname: {}\nOS: {}\n\n*Logged in at: {}*",
        username.trim_matches('"').to_string(),
        hostname.unwrap_or_else(|_| "not found".to_string()).trim_matches('"').to_string(),
        os.trim_matches('"').to_string(),
        timestamp.trim_matches('"').to_string()
    ).as_str();
    
    webhook.send(&Message::new(|message| message.content(message_content))).await.unwrap();

    exit(0);
}
 
extern crate chrono;
use chrono::Utc;

use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug)]
enum InternalError {
    URL(String),
    LOGIN
}
use std::net::ToSocketAddrs;

fn main() {

    // computes the path of the config file
    let program_path = std::env::args().nth(0).unwrap();
    let program_name = program_path.split(std::path::MAIN_SEPARATOR).collect::<Vec<_>>().pop().unwrap();
    let config_path = format!("{}config.json",program_path.replace(program_name,""));
    
    // gets the json from the config and checks the format of the config.json
    let config = match parse_config(config_path) {
        Err(_) => {println!("{} Problem while parsing the config.json. Check your config!\n Terminated",Utc::now().format("[%F %T]")); return;},
        Ok(value) => value,
    };

    // start to check frequently the given domains. Returns if something runs wrong
    match frequently_check(config.refresh_time, config.user,config.password,config.domains) {
        Ok(()) => println!("{} Exited Tool", Utc::now().format("[%F %T]")),
        Err(InternalError::LOGIN) => println!("{} Problem with host login!\n Terminated", Utc::now().format("[%F %T]")),
        Err(InternalError::URL(domain)) => println!("{} Problem with set domains. Check if you are the owner of the domain {}!\n Terminated", Utc::now().format("[%F %T]"), domain),
    }
}

/// Checks in `interval` (minutes) the domains `domain`. 
/// If the current ip don't show on the domains id the record will updated
fn frequently_check(interval: u32,username: String, password: String, domains: Vec<String>) -> Result<(),InternalError> {
    let dns_manager = match hoster_tools::hosters::onyxhosting::DNSManager::new(username.as_str(), password.as_str()) {
                        Err(_) => { return Err(InternalError::LOGIN) },
                        Ok(manager) => manager,
                    };

    println!("{} Startup tool.",Utc::now().format("[%F %T]"));
    loop {
        for domain in domains.iter() {
            let target_ip = get_current_ip(domain)?;
            let own_ip = get_own_ip();
            if !own_ip.contains(target_ip.as_str()) {
                dns_manager.add_dns_record(domain, "", "A", &own_ip).map_err(|_|InternalError::LOGIN)?;
                println!("{} Domain {} shows on {} but the current ip address is {}. Updated!",Utc::now().format("[%F %T]"), domain, target_ip, own_ip);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis((interval*60*1000) as u64))
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    user: String,
    password: String,
    domains: Vec<String>,
    refresh_time: u32,
}

fn parse_config(file_path: String) -> std::io::Result<Config> {
    // Open the file in read-only mode with buffer.;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let config: Config = serde_json::from_reader(reader)?;

    for domain in config.domains.iter() {
        if check_url(&domain).is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable,format!("Forbidden characters in domain {}", domain)));
        }
    }
    Ok(config)
}

fn get_own_ip() -> String {
    let resp = reqwest::blocking::get("https://api.ipify.org/").unwrap().text().unwrap();
    resp
}

fn get_current_ip(domain: &String) -> Result<String,InternalError> {
    let addr = format!("{}:80",domain).to_socket_addrs().unwrap().nth(0).ok_or(InternalError::URL(format!("Can't resolve {}",domain)))?.ip();
    if !addr.is_ipv4() {
        return Err(InternalError::URL(format!("This tool can just handle ipv4.")))
    }
    let addr_str = format!("{}",addr);
    Ok(addr_str)
}

/// The URL isn't allowed to contain https://. 
/// So this function will return an Err, if the url contains forbidden characters.
fn check_url(url: &str) -> Result<(),InternalError> {
    if url.contains(",") || url.contains(":") || url.contains("/") {
        return Err(InternalError::URL(String::from(url)));
    } 

    Ok(())
}
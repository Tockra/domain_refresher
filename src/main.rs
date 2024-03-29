extern crate chrono;
extern crate regex;

use chrono::Utc;
use regex::Regex;

use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug)]
enum InternalError {
    URL(String),
    LOGIN
}
use std::net::ToSocketAddrs;


#[tokio::main]
async fn main() {

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
    match frequently_check(config.refresh_time, config.user,config.password,config.domains).await {
        Ok(()) => println!("{} Exited Tool", Utc::now().format("[%F %T]")),
        Err(InternalError::LOGIN) => println!("{} Problem with host login!\n Terminated", Utc::now().format("[%F %T]")),
        Err(InternalError::URL(domain)) => println!("{} Problem with set domains. Check if you are the owner of the domain {}!\n Terminated", Utc::now().format("[%F %T]"), domain),
    }
}

/// Checks in `interval` (minutes) the domains `domain`. 
/// If the current ip don't show on the domains id the record will updated
async fn frequently_check(interval: u32,username: String, password: String, domains: Vec<String>) -> Result<(),InternalError> {
    let _dns_manager = match hoster_tools::hosters::onyxhosting::DNSManager::new(username.as_str(), password.as_str()) {
                        Err(_) => { return Err(InternalError::LOGIN) },
                        Ok(manager) => manager,
                    };

    println!("{} Startup tool.",Utc::now().format("[%F %T]"));
    let mut last_ip = String::from("127.0.0.1");
    let mut counter = 0;
    loop {
        for domain in domains.iter() {
            let target_ip = get_current_ip(domain)?;
            let own_ip = get_own_ip().await;
            if own_ip != "" {
                last_ip = own_ip;
            } else {
                counter +=1;

                if counter == 3 {
                    panic!("Third Problem with the ip service. Please check your internet connection and restart this program.")
                }
            }

            let re_ip = Regex::new(r"^\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}$").unwrap();
            if !last_ip.contains(target_ip.as_str()) && re_ip.is_match(&last_ip) {
                // temporary deactivated until it works for one month TODO
                //dns_manager.add_dns_record(domain, "", "A", &last_ip).map_err(|_|InternalError::LOGIN)?;
                println!("{} Domain {} shows on {} but the current ip address is {}. Updated!",Utc::now().format("[%F %T]"), domain, target_ip, last_ip);
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

async fn get_own_ip() -> String {
    let result = external_ip::get_ip();
    let value : Option<IpAddr> = result.await;
    return match value {
        Some(x) => x.to_string(),
        None => String::from("")
    };
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

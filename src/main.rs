use serde_json::Value;

enum InternalError {
    URL(String),
    LOGIN
}

fn main() {

    // computes the path of the config file
    let program_path = std::env::args().nth(0).unwrap();
    let program_name = program_path.split(std::path::MAIN_SEPARATOR).collect::<Vec<_>>().pop().unwrap();
    let config_path = format!("{}{}config.json",program_path.replace(program_name,""),std::path::MAIN_SEPARATOR);
    
    // gets the json from the config and checks the format of the config.json
    let config = match parse_config(config_path) {
        Err(_) => {println!("Problem while parsing the config.yml. Check your config!\n Terminated"); return;},
        Ok(value) => value,
    };

    // start to check frequently the given domains. Returns if something runs wrong
    match frequently_check(config["refresh_time"].as_u64().unwrap(), String::from(""),String::from(""),vec![]) {
        Ok(()) => println!("Exited Tool"),
        Err(InternalError::LOGIN) => println!("Problem with host login!\n Terminated"),
        Err(InternalError::URL(domain)) => println!("Problem with set domains. Check if you are the owner of the domain {}!\n Terminated", domain),
    }
}

/// Checks in `interval` (minutes) the domains `domain`. 
/// If the current ip don't show on the domains id the record will updated
fn frequently_check(interval: u64,username: String, password: String, domains: Vec<String>) -> Result<(),InternalError> {
    let dns_manager = match hoster_tools::hosters::onyxhosting::DNSManager::new(username.as_str(), password.as_str()) {
                        Err(_) => { return Err(InternalError::LOGIN) },
                        Ok(manager) => manager,
                    };

    
    loop {
        for domain in domains.iter() {
            if !get_own_ip().contains(&get_current_ip(domain)) {
                // Update entry
            }
        }
        std::thread::sleep_ms(interval as u32*60*1000);
    }
    Ok(())
}

fn parse_config(file_path: String) -> Result<Value,std::fmt::Error> {

    // if !config["refresh_time"].is_number || config["refresh_time"].is_
    unimplemented!();
}

fn get_own_ip() -> String {
    unimplemented!();
}

fn get_current_ip(domain: &String) -> String {
    unimplemented!();
}

fn check_url(url: &str) -> Result<(),InternalError> {
    if url.contains(",") || url.contains(":") || url.contains("/") {
        return Err(InternalError::URL(String::from(url)));
    } 

    Ok(())
}
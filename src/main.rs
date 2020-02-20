use serde_json::Value;

enum InternalError {
    Config,
    URL,
    LOGIN
}

fn main() {

    // gets the json from the config and checks the format of the config.json

    // start to check frequently the given domains. Returns if something runs wrong

}

fn frequently_check(interval: u32,domains: Vec<String>) -> Result<(),InternalError> {
    unimplemented!();
}

fn parse_config(file_path: &str) -> Result<Value,InternalError> {
    unimplemented!();
}

fn get_own_ip() -> String {
    unimplemented!();
}

fn get_current_ip(domain: &str) -> String {
    unimplemented!();
}

fn check_url(url: &str) -> Result<(),InternalError> {
    if url.contains(",") || url.contains(":") || url.contains("/") {
        return Err(InternalError::URL);
    } 

    Ok(())
}
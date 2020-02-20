use serde_json::Value;
fn main() {
    println!("Hello world");
}


fn check_url(url: &str) -> Result<(),String> {
    if url.contains(",") || url.contains(":") || url.contains("/") {
        return Err("Falsches Format der URL");
    } 

    Ok(())
}
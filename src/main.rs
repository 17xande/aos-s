
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}/rest/v8/ports/{}/mac-table", "192.168.16.72", "8");
    let resp = reqwest::blocking::get(url)?
        .json::<serde_json::Value>();
    println!("{:#?}", resp);
    Ok(())
}


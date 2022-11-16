#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("config.json").unwrap();
    let js = serde_json::from_str::<serde_json::Value>(&text).unwrap();
    let url = js["address"].to_string();
    let res = reqwest::get(url.replace("\"", "")).await.unwrap();
    let body = res.text().await.unwrap();
    println!("{}", body);

    Ok(())
}
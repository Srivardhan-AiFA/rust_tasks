use reqwest::Certificate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ca_pem = std::fs::read("src/keys/server.pem")?;
    let ca = Certificate::from_pem(&ca_pem)?;

    let client = reqwest::Client::builder()
        .add_root_certificate(ca)
        .build()?;

    let res = client.get("https://0.0.0.0:3000").send().await?;

    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await?);

    Ok(())
}

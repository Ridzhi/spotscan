use spotscan::{
    prelude::*,
    spot
};
use reqwest;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let schedule: Schedule = client.get("https://atlanticspot.ru/api/booking/times.php")
        .query(&[("bookingDate", "30.09.2025")])
        .send()
        .await?
        .json::<spot::Response>()
        .await?
        .into();

    println!("{schedule:#?}");
    Ok(())
}
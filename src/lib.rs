use crate::models::Components;

pub mod models;
pub mod cli;

pub const BASE_URL: &str = "https://www.cloudflarestatus.com/api/v2";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_info(name: &String) -> Result<()> {
    let resp = reqwest::get(format!("{}/components.json", BASE_URL))
            .await
            .unwrap()
            .json::<Components>()
            .await
            .unwrap();

    if let Some(cmp) = resp.components.iter().find(|cmp| cmp.name.starts_with(name)).cloned() {
        println!("{}", cmp);
    } else {
        println!("Nothing found for {}", name);
    }

    Ok(())
}
use summary::Summary;

pub mod summary;
pub mod cli;

pub const BASE_URL: &str = "https://www.cloudflarestatus.com/api/v2";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_summary(name: &String) -> Result<()> {
    let resp = reqwest::get(format!("{}/summary.json", BASE_URL))
            .await
            .unwrap()
            .json::<Summary>()
            .await
            .unwrap();

    let moscow_component  = resp
            .components
            .iter()
            .find(|cmp| cmp.name.starts_with(name));

    println!("{:#?}", moscow_component);

    Ok(())
}
use std::fs;

use fantoccini::{cookies::Cookie, Client};

#[derive(serde::Deserialize)]
struct CookieData {
    #[serde(rename="Name raw")]
    pub name: String,
    #[serde(rename="Content raw")]
    pub content: String,
}

pub async fn load_cookies<'a>(c: &Client) -> Result<(), fantoccini::error::CmdError> {
    const COOKIE_PATH: &str = "cookies.json";

    let cookie_file = fs::read_to_string(COOKIE_PATH)
        .expect("Cookie file not found.");
    
    let cookie_data: Vec<CookieData> = serde_json::from_str(&cookie_file)
        .expect("Couldn't read cookie json.");
    
    for cd in cookie_data {
        c.add_cookie(Cookie::new(cd.name, cd.content)).await?;
    }

    Ok(())
}

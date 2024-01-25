pub mod visma_navigation;
pub mod bulking;

use std::{path::Path, thread::sleep, time::Duration}; use bulking::article::{add_articles, Article};
use fantoccini::ClientBuilder;
use tokio;
use visma_navigation::init_site;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    init_site(&c).await?;

    let article = Article {
        number: String::from("123123"),
        name: String::from("Lampjävel"),
        image_path: &Path::new("./article/Untitled.jpg"),
        price_vat: 259.2
    };

    let art_vec = vec![&article];
    add_articles(&c, &art_vec).await?;

    sleep(Duration::from_secs(10));
    c.close().await
}

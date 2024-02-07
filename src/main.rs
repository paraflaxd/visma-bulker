pub mod visma_navigation;
pub mod bulking;
pub mod generator;

use std::path::Path; use bulking::article::add_articles;
use fantoccini::ClientBuilder;
use generator::article::gen_articles;
use tokio;
use visma_navigation::init_site;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    init_site(&c).await?;

    let articles = gen_articles(Path::new("./article")).await;
    add_articles(&c, &articles).await?;

    c.close().await
}

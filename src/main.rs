pub mod visma_navigation;
pub mod bulking;

use std::{thread::sleep, time::Duration}; use fantoccini::ClientBuilder;
use tokio;
use visma_navigation::{goto_article_menu, init_site};

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native().connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

    init_site(&c).await?;

    goto_article_menu(&c).await?;

    sleep(Duration::from_secs(10));
    c.close().await
}

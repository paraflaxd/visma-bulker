pub mod cookie_loading;

use fantoccini::{Client, Locator};

use self::cookie_loading::load_cookies;

const WEBSHOP_MENU: Locator<'static> = Locator::XPath("/html/body/div[2]/nav/div[1]/ul[1]/li[2]/a");
const ARTICLE_BUTTON: Locator<'static> = Locator::XPath("//*[@id=\"admin-settings-article-register\"]");

pub async fn init_site(c: &Client) -> Result<(), fantoccini::error::CmdError> {
    // Fix InvalidCookieDomain error
    c.goto("https://www.yourvismawebsite.com/admin").await?;

    let _ = load_cookies(&c).await?;
    c.goto("https://www.yourvismawebsite.com/admin").await?;

    Ok(())
}

pub async fn goto_article_menu(c: &Client) -> Result<(), fantoccini::error::CmdError> {
    let webshop_menu = c.find(WEBSHOP_MENU).await?;
    webshop_menu.click().await?;

    let article_button = c.find(ARTICLE_BUTTON).await?;
    article_button.click().await?;

    Ok(())
}

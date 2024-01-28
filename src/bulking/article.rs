use std::{env, path::{Path, PathBuf}};

use fantoccini::{Client, Locator};

use crate::visma_navigation::goto_article_menu;

pub struct Article<'a> {
    pub number: String,
    pub name: String,
    pub price_vat: f32,
    pub image_path: &'a Path
}

const EDITOR_BTN: Locator<'static> = Locator::XPath("//*[@id=\"create-article\"]");
const NUMBER_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"articleno\"]");
const NAME_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"SV_Name\"]");
const PRICE_VAT_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"gross-price\"]");
const IMG_UPLOAD: Locator<'static> = Locator::XPath("/html/body/div[9]/div/div/div[2]/form/div[1]/div/div[2]/div[1]/div/div[2]/div/div[1]/div/input[3]");
const SAVE_BTN: Locator<'static> = Locator::XPath("/html/body/div[9]/div/div/div[4]/div/div/div/button[2]");

pub async fn add_articles<'a>(c: &Client, articles: &Vec<&Article<'a>>) -> Result<(), fantoccini::error::CmdError> {
    goto_article_menu(&c).await?;
    
    c.find(EDITOR_BTN).await?.click().await?;

    for a in articles {
        send_keys(c, NUMBER_INPUT, &a.number).await?;
        send_keys(c, NAME_INPUT, &a.name).await?;
        send_keys(c, PRICE_VAT_INPUT, &a.price_vat.to_string()).await?;
        upload_image(c, &a.image_path).await?;
    }
    
    Ok(())
}

async fn send_keys<'a>(c: &Client, locator: Locator<'a>, text: &str) -> Result<(), fantoccini::error::CmdError> {
    c.find(locator).await?.send_keys(&text).await?;
    Ok(())
}

// TODO: Fix file upload (image gets displayed, not uploaded - trigger jquery event???)
async fn upload_image(c: &Client, image: &Path) -> Result<(), fantoccini::error::CmdError> {
    let img_upload = c.find(IMG_UPLOAD).await?;

    let absolute_path: String = match image.is_absolute() {
        true => image.to_owned(),
        false => PathBuf::from(env::current_dir()?.join(image)).to_owned(),
    }.to_str().expect("Invalid image path").to_string();

    img_upload.send_keys(&absolute_path).await.expect("Image upload failed");

    c.find(SAVE_BTN).await?.click().await?;

    Ok(())
}

use std::{env, path::{Path, PathBuf}};
use async_recursion::async_recursion;
use fantoccini::{Client, Locator};

use crate::visma_navigation::goto_article_menu;

#[derive(Clone, Debug)]
pub struct Article {
    pub number: String,
    pub name: String,
    pub price_vat: f32,
    pub image_path: PathBuf
}

const EDITOR_BTN: Locator<'static> = Locator::XPath("//*[@id=\"create-article\"]");
const VISIBLE_ART_MENU: Locator<'static> = Locator::Css("#dialog_articleregister_main:not([style*=\"display: none\"])");
const NUMBER_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"articleno\"]");
const NAME_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"SV_Name\"]");
const PRICE_VAT_INPUT: Locator<'static> = Locator::XPath("//*[@id=\"gross-price\"]");
const IMG_UPLOAD: Locator<'static> = Locator::XPath("/html/body/div[9]/div/div/div[2]/form/div[1]/div/div[2]/div[1]/div/div[2]/div/div[1]/div/input[3]");
const SAVE_BTN: Locator<'static> = Locator::XPath("/html/body/div[9]/div/div/div[4]/div/div/div/button[2]");
const RANDOM_FIELD: Locator<'static> = Locator::XPath("//*[@id=\"summary\"]");
const UPLOAD_FILE_INPUT_UPLOADED: Locator<'static> = Locator::Css("#uploadedImageFile:not([value=\"40eea01b-02b1-4766-80c7-8ae2b08b802c\"]):not([value=\"00000000-0000-0000-0000-000000000000\"])");

#[async_recursion]
pub async fn add_articles(c: &Client, articles: &Vec<Article>) -> Result<(), fantoccini::error::CmdError> {
    goto_article_menu(c).await?;
    
    for (pos, a) in articles.iter().enumerate() {
        match add_article(c, a).await {
            Ok(_) => (),
            Err(_) => {
                c.refresh().await?;
                add_articles(c, &articles[pos..].to_vec()).await?;
                return Ok(());
            }
        };
    }

    Ok(())
}

async fn add_article(c: &Client, article: &Article) -> Result<(), fantoccini::error::CmdError> {
        c.wait().for_element(VISIBLE_ART_MENU).await?;
        c.wait().for_element(EDITOR_BTN).await?;
        c.find(EDITOR_BTN).await?.click().await?;
        send_keys(c, NUMBER_INPUT, &article.number).await?;
        send_keys(c, NAME_INPUT, &article.name).await?;
        send_keys(c, PRICE_VAT_INPUT, &article.price_vat.to_string()).await?;
        upload_image(c, &article.image_path).await?;

        // Allows the site to trigger some events, upload won't work otherwise
        c.find(RANDOM_FIELD).await?.send_keys("").await?;

        c.wait().for_element(UPLOAD_FILE_INPUT_UPLOADED).await?;
        c.find(SAVE_BTN).await?.click().await?;

        c.wait().for_element(VISIBLE_ART_MENU).await?;

        Ok(())
}

async fn send_keys<'a>(c: &Client, locator: Locator<'a>, text: &str) -> Result<(), fantoccini::error::CmdError> {
    c.find(locator).await?.send_keys(&text).await?;
    Ok(())
}

async fn upload_image(c: &Client, image: &Path) -> Result<(), fantoccini::error::CmdError> {
    let absolute_path: String = match image.is_absolute() {
        true => image.to_owned(),
        false => PathBuf::from(env::current_dir()?.join(image)).to_owned(),
    }.to_str().expect("Invalid image path").to_string();

    let img_upload = c.find(IMG_UPLOAD).await?;
    img_upload.send_keys(&absolute_path).await.expect("Image upload failed");

    let script = r#"
        $('#articleDragDropImage1').find('.image-upload-input').trigger("change");
        "#;

    c.execute(&script, vec![]).await?;

    c.wait().for_element(UPLOAD_FILE_INPUT_UPLOADED).await?;
    Ok(())
}

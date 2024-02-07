use std::{fs::read_dir, path::{Path, PathBuf}};

use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::bulking::article::Article;

const FILE_TYPES: &[&str] = &["png", "jpg"];
const ADJECTIVES: &[&str] = &["fin", "modern", "fantastisk", "bra", "lyxig", "gedigen", "vacker", "oemotståndlig", "katastrofal", "helt ny", "obegagnad", "stor", "liten", "neutral", "elegant", "snygg", "stilig"];

pub async fn gen_articles<'a>(img_path: &Path) -> Vec<Article> {
    let paths: Vec<PathBuf> = read_dir(img_path)
        .expect("Couldn't find image directory")
        .into_iter()
        .map(|p| p.unwrap().path().to_owned())
        .filter(|p| FILE_TYPES.contains(&p.extension().unwrap().to_str().unwrap()))
        .collect();

    let mut rng = thread_rng();
    let mut articles: Vec<Article> = Vec::new();
    for p in paths {
        let art = Article {
            number:rng.gen_range(10000..19000).to_string(),
            name: get_name(2),
            price_vat: rng.gen_range(350.0..1500.0),
            image_path: p
        };
        articles.push(art);
    }
    articles
}

fn get_name(combo_count: i8) -> String {
    let mut combo = String::new();
    for _ in 0..combo_count {
        combo.push_str(*ADJECTIVES.choose(&mut thread_rng()).unwrap());
        combo.push_str(" ");
    }
    combo.push_str("lampskärm");
    combo
}

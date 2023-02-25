use std::error::Error;

use serde::Deserialize;
use colour::{dark_green, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles = serde_json::from_str(&response)?;
    
    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);

    }
}


fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=2cb60046470648819d5b0c6e115539d6";
    let articles = get_articles(url)?;
    
    render_articles(&articles);
    Ok(())
}

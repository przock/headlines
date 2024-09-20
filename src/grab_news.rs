use std::collections::BTreeMap;
use chrono::Utc;
use newsapi::api::NewsAPIClient;
use newsapi::constants::{Language, SortMethod};
use newsapi::payload::article::Articles;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::env;

#[derive(Serialize, Deserialize)]
struct ArticleInfo {
    source_name: String,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: String,
    url_to_image: Option<String>,
    published_at: String,
    content: Option<String>,
}

pub fn news_get_as_json() -> Result<BTreeMap<String, Value>, Box<dyn std::error::Error>> {

    let key = env::var("NEWSAPI_KEY")?;

    let start_timestamp = Utc::now() - chrono::Duration::seconds(604800);
    let end_timestamp = Utc::now();

    let mut client = NewsAPIClient::new(key);

    client
        .language(Language::English)
	.with_sources(String::from("bbc-news, abc-news"))
        .from(&start_timestamp)
        .to(&end_timestamp)
        .sort_by(SortMethod::Popularity)
        .everything();

    let articles = client.send_sync::<Articles>()?;
    let article_data = articles.articles;
    let mut articles_map: BTreeMap<String, Value> = BTreeMap::new();
    

    for (index, article) in article_data.iter().enumerate() {
	let article_content = article.content.clone();

	// this is a special case because sometimes the content is truncated
	let content_value = if let Some(article_content) = article_content {
	    if article_content.ends_with("chars]") {
		format!("{} {}", &article_content, "...Continue reading at the source:",)
	    } else {
		article_content
	    }
	} else {
	    "No content found.".to_string()
	};

	let article_json = json!({
	    "title": article.title,
	    "author": article.author,
	    "content": content_value,
	    "description": article.description,
	    "source_name": article.source.name,
	    "url": article.url,
	    "url_to_image": article.url_to_image,
	    "published_at": article.published_at,
	});

	let article_data_as_json_value = serde_json::to_value(article_json)?;
	articles_map.insert(format!("Article #{}", index), article_data_as_json_value);
    }

    Ok(articles_map)
}

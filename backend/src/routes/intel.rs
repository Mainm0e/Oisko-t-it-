use axum::{Json, extract::Query, http::StatusCode, response::IntoResponse};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct IntelQuery {
    pub url: String,
}

#[derive(Serialize)]
pub struct CompanyIntel {
    pub company_name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
}

pub async fn get_company_intel(Query(params): Query<IntelQuery>) -> impl IntoResponse {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()
        .unwrap_or_default();

    let resp = match client.get(&params.url).send().await {
        Ok(res) => res,
        Err(_) => return (StatusCode::BAD_REQUEST, "Failed to reach URL").into_response(),
    };

    let body = match resp.text().await {
        Ok(text) => text,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read body").into_response();
        }
    };

    let document = Html::parse_document(&body);

    // 1. Company Name (Title or og:site_name)
    let company_name = extract_meta(&document, "property", "og:site_name")
        .or_else(|| extract_title(&document))
        .map(|s| s.trim().split('|').next().unwrap_or(&s).trim().to_string());

    // 2. Description (og:description or meta description)
    let description = extract_meta(&document, "property", "og:description")
        .or_else(|| extract_meta(&document, "name", "description"));

    // 3. Logo (og:image or favicon or apple-touch-icon)
    let logo_url = extract_meta(&document, "property", "og:image")
        .or_else(|| extract_link_rel(&document, "apple-touch-icon"))
        .or_else(|| extract_link_rel(&document, "icon"))
        .or_else(|| extract_link_rel(&document, "shortcut icon"))
        .map(|s| make_absolute_url(&params.url, &s));

    Json(CompanyIntel {
        company_name,
        description,
        logo_url,
    })
    .into_response()
}

fn extract_meta(doc: &Html, attr: &str, value: &str) -> Option<String> {
    let selector = Selector::parse(&format!("meta[{}='{}']", attr, value)).ok()?;
    doc.select(&selector)
        .next()?
        .value()
        .attr("content")
        .map(|s| s.to_string())
}

fn extract_title(doc: &Html) -> Option<String> {
    let selector = Selector::parse("title").ok()?;
    doc.select(&selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(""))
}

fn extract_link_rel(doc: &Html, rel: &str) -> Option<String> {
    let selector = Selector::parse(&format!("link[rel='{}']", rel)).ok()?;
    doc.select(&selector)
        .next()?
        .value()
        .attr("href")
        .map(|s| s.to_string())
}

fn make_absolute_url(base: &str, relative: &str) -> String {
    if relative.starts_with("http") {
        return relative.to_string();
    }

    let base_url = reqwest::Url::parse(base).ok();
    if let Some(base_url) = base_url {
        if let Ok(abs) = base_url.join(relative) {
            return abs.to_string();
        }
    }
    relative.to_string()
}

use anyhow::{anyhow, Result};
use reqwest::{Client};
use serde::Deserialize;
use serde_json::Value;
use log::info;

#[derive(Debug, Clone)]
pub struct MovieMetadata {
    pub title: Option<String>,
    pub original_title: Option<String>,
    pub year: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub kinopoisk_id: i64,
    pub kinopoisk_url: String,
    pub raw_data: Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct KinoMovie {
    name: Option<String>,
    alternative_name: Option<String>,
    year: Option<i32>,
    description: Option<String>,
    poster: Option<KinoPoster>,      // ← Объект, а не строка
}

#[derive(Deserialize, Debug)]
struct KinoPoster {
    url: Option<String>,
    #[serde(rename = "previewUrl")]
    preview_url: Option<String>,
}

#[derive(Clone, Debug)]
pub struct KinoClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl KinoClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn find_by_id(&self, id: i64, kinopoisk_url: &str) -> Result<MovieMetadata> {
        let url = format!("{}/{}", self.base_url, id);

        let response = self.client
            .get(&url)
            .header("X-API-KEY", &self.api_key)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("API вернул ошибку {}: {}", status, body));
        }

        let raw_data: Value = response.json().await?;
        info!("raw_data: {}", serde_json::to_string_pretty(&raw_data)?);

        // Десериализуем напрямую в KinoMovie (НЕ в обёртку с Vec)
        let movie: KinoMovie = serde_json::from_value(raw_data.clone())
            .map_err(|e| anyhow!("Ошибка десериализации: {}. JSON: {}", e, raw_data))?;

        info!("Movie: {:?}", movie);

        // Извлекаем URL постера из вложенного объекта
        let poster_url = movie.poster
            .and_then(|p| p.url.or(p.preview_url));

        // Извлекаем рейтинг Кинопоиска
        Ok(MovieMetadata {
            title: movie.name,
            original_title: movie.alternative_name,
            year: movie.year,
            description: movie.description,
            poster_url,
            kinopoisk_id: id,
            kinopoisk_url: kinopoisk_url.to_string(),
            raw_data,
        })
    }
}

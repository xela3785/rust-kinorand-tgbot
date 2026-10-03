use anyhow::{anyhow, Result};
use regex::Regex;

pub fn extract_id(url: &str) -> Result<i64> {
    let re = Regex::new(r"kinopoisk\.ru/film/(\d+)")?;

    match re.captures(url) {
        Some(caps) => {
            let id_str = &caps[1];
            id_str.parse::<i64>()
                .map_err(|_| anyhow!("Failed to parse ID from URL: {}", url))
        }
        None => Err(anyhow!("Failed to match ID from URL: {}", url)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_kinopoisk_id() {
        assert_eq!(
            extract_id("https://www.kinopoisk.ru/film/258687/").unwrap(),
            258687
        );
        assert_eq!(
            extract_id("https://www.kinopoisk.ru/film/326/?utm_referrer=organic.kinopoisk.ru").unwrap(),
            326
        );
        assert_eq!(
            extract_id("https://www.kinopoisk.ru/film/435/?utm_referrer=organic.kinopoisk.ru").unwrap(),
            435
        );
        assert!(
            extract_id("https://www.example.com/film/435/").is_err()
        );
        assert!(
            extract_id("https://www.kinopoisk.ru/").is_err()
        );
    }
}
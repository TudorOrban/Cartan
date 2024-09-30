

pub async fn fetch_url(url: String) -> Result<String, String> {
    if url.is_empty() {
        return Ok(String::from(""));
    }

    reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}
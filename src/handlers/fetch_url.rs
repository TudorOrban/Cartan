

pub async fn fetch_url(url: String) -> Result<String, String> {
    if url.is_empty() {
        return Ok(String::from(""));
    }
    
    
    match surf::get(url).recv_string().await {
        Ok(text) => Ok(text),
        Err(e) => Err(e.to_string()),
    }
}
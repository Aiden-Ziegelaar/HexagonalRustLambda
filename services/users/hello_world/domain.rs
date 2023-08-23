pub async fn hello_world_core (who: Option<&str>) -> Result<String, &'static str> {
    Ok(format!("Hello, {}!", who.unwrap_or("world")))
}
use error::HexagonalError;

pub async fn hello_world_core(who: Option<&str>) -> Result<String, HexagonalError> {
    Ok(format!("Hello, {}!", who.unwrap_or("world")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_world_core() {
        let result = hello_world_core(None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn test_hello_world_core_with_who() {
        let result = hello_world_core(Some("John")).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, John!");
    }
}

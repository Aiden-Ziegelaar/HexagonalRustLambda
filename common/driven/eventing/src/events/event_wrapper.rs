use serde::Deserialize;

#[derive(Deserialize)]
pub struct EventWrapper {
    pub version: u32,
    pub event_type: String,
}

impl EventWrapper {
    pub fn new(version: u32, event_type: String) -> Self {
        Self {
            version,
            event_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_event_wrapper_valid() {
        let input = r#"{
            "version": 1,
            "event_type": "test"
        }"#;
        let result: Result<EventWrapper, serde_json::Error> = serde_json::from_str(input);
        assert!(result.is_ok());
        let result_value = result.unwrap();
        assert_eq!(result_value.version, 1);
        assert_eq!(result_value.event_type, "test");
    }

    #[test]
    fn test_event_wrapper_extra_fields() {
        let input = r#"{
            "version": 1,
            "event_type": "test",
            "extra_field": "extra"
        }"#;
        let result: Result<EventWrapper, serde_json::Error> = serde_json::from_str(input);
        assert!(result.is_ok());
        let result_value = result.unwrap();
        assert_eq!(result_value.version, 1);
        assert_eq!(result_value.event_type, "test");
    }

    #[test]
    fn test_event_wrapper_missing_fields() {
        let input = r#"{
            "version": 1
        }"#;
        let result: Result<EventWrapper, serde_json::Error> = serde_json::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_wrapper_invalid_version() {
        let input = r#"{
            "version": "1",
            "event_type": "test"
        }"#;
        let result: Result<EventWrapper, serde_json::Error> = serde_json::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_wrapper_invalid_event_type() {
        let input = r#"{
            "version": 1,
            "event_type": 1
        }"#;
        let result: Result<EventWrapper, serde_json::Error> = serde_json::from_str(input);
        assert!(result.is_err());
    }
}

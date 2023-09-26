use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_json::json;

use jsonschema::{Draft, JSONSchema};

pub static EVENT_WRAPPER_SCHEMA: LazyLock<JSONSchema> = LazyLock::new(|| {
    let schema = json!({
        "type": "object",
        "properties": {
            "version": {
                "type": "number"
            },
            "event_type": {
                "type": "string"
            }
        },
        "required": [
            "version",
            "event_type"
        ],
        "additionalProperties": true
    });
    JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema)
        .unwrap()
});

#[derive(Serialize, Deserialize)]
pub struct EventWrapper {
    pub version: u32,
    pub event_type: String,
}

impl EventWrapper {
    pub fn new(version: u32, event_type: String) -> Self {
        Self { version, event_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
}
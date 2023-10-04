#[macro_export]
macro_rules! http_payload_decoder {
    ($typedef:ident, $schema:ident, $payload:ident) => {{
        if $payload.is_none() {
            let err = error::HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "Payload is required".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }

        let payload_str = $payload.clone().unwrap();
        let payload_json_result = serde_json::from_str::<serde_json::Value>(&payload_str);
        if payload_json_result.is_err() {
            let err = error::HexagonalError {
                error: error::HexagonalErrorCode::BadInput,
                message: "Payload is not valid JSON".to_string(),
                trace: "".to_string(),
            };
            return Ok(err.compile_to_http_response());
        }
        let payload_json = payload_json_result.unwrap();

        let result = $schema.validate(&payload_json);
        match result {
            Ok(_) => {}
            Err(e) => {
                let err_msg = e
                    .enumerate()
                    .fold("".to_string(), |acc, x| format!("{}, {}", acc, x.1));

                let err = error::HexagonalError {
                    error: error::HexagonalErrorCode::BadInput,
                    message: err_msg,
                    trace: "".to_string(),
                };
                return Ok(err.compile_to_http_response());
            }
        }
        serde_json::from_str::<$typedef>(&payload_str).unwrap()
    }};
}

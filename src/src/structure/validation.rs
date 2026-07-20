use crate::structure::{Request, ResponseData, ValidationResult};

pub fn validate(request: &Request, response: &ResponseData) -> ValidationResult {
    let expect = match &request.expect {
        Some(v) => v,
        None => {
            return ValidationResult {
                status: true,
                contains: true,
                not_contains: true,
            };
        }
    };

    ValidationResult {
        status: expect
            .status
            .map(|s| response.status.as_u16() == s)
            .unwrap_or(true),

        contains: expect
            .contains
            .as_ref()
            .map(|v| response.body.contains(v))
            .unwrap_or(true),

        not_contains: expect
            .not_contains
            .as_ref()
            .map(|v| !response.body.contains(v))
            .unwrap_or(true),
    }
}

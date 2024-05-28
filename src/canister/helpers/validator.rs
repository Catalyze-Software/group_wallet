use types::{Error, ValidateField, ValidationResponse, ValidationType};

use super::str::str_len;

pub struct Validator {
    fields: Vec<ValidateField>,
}

impl Validator {
    pub fn new(fields: Vec<ValidateField>) -> Self {
        Validator { fields }
    }

    pub fn validate(&self) -> Result<(), Error> {
        let mut errors: Vec<ValidationResponse> = vec![];

        self.fields.iter().for_each(|f| {
            if let Err(err) = Self::validate_field(f) {
                errors.push(err);
            }
        });

        if !errors.is_empty() {
            return Err(Error::validation_response(errors));
        }

        Ok(())
    }

    fn validate_field(validation_field: &ValidateField) -> Result<(), ValidationResponse> {
        let ValidateField(validation_type, field) = validation_field;

        use ValidationType::*;
        match validation_type {
            None => Ok(()),
            StringLength(value, min, max) => Self::validate_string_length(value, min, max, field),
            Count(value, min, max) => Self::validate_count(value, min, max, field),
        }
    }

    fn validate_string_length(
        value: &str,
        min: &usize,
        max: &usize,
        field: &String,
    ) -> Result<(), ValidationResponse> {
        if str_len(value) < *min {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Minimum required length is {}", min),
            });
        }
        if str_len(value) > *max {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Maximum length is {}", max),
            });
        }

        Ok(())
    }

    fn validate_count(
        value: &usize,
        min: &usize,
        max: &usize,
        field: &String,
    ) -> Result<(), ValidationResponse> {
        if value < min {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Minimum size length is {}", min),
            });
        }
        if value > max {
            return Err(ValidationResponse {
                field: field.to_string(),
                message: format!("Maximum size is {}", max),
            });
        }

        Ok(())
    }
}

use juniper::{self, Object, Value};
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

pub struct ValidationErrorsWrapper(pub ValidationErrors);

pub trait FromValidationErrors {
    fn from_validation_errors(errors: ValidationErrors) -> Self;
}

fn juniper_value_from_serde_value(value: &serde_json::Value) -> juniper::Value {
    match value {
        serde_json::value::Value::Null => Value::Null,
        serde_json::value::Value::Bool(x) => Value::scalar(*x),
        serde_json::value::Value::Number(x) if x.is_f64() => {
            Value::scalar(x.as_f64().unwrap_or(0.0) as f64)
        }
        serde_json::value::Value::Number(x) => Value::scalar(x.as_i64().unwrap_or(0) as i32),
        serde_json::value::Value::String(x) => Value::scalar(x.to_owned()),
        serde_json::value::Value::Array(xs) => juniper::Value::list(
            xs.into_iter()
                .map(|v| juniper_value_from_serde_value(v))
                .collect(),
        ),
        serde_json::value::Value::Object(map) => {
            let jobj = juniper::Object::with_capacity(map.len());
            juniper::Value::object(map.into_iter().fold(jobj, |mut jobj, (k, v)| {
                jobj.add_field(k, juniper_value_from_serde_value(v));
                jobj
            }))
        }
    }
}

fn juniper_value_from_validation_error(error: &ValidationError) -> juniper::Value {
    let mut jobj = juniper::Object::with_capacity(3);
    jobj.add_field("code", Value::scalar(error.code.as_ref().clone()));
    if let Some(message) = &error.message {
        jobj.add_field("message", Value::scalar(message.as_ref().clone()));
    }
    let params = Object::with_capacity(error.params.len());
    jobj.add_field(
        "params",
        Value::object(
            error
                .params
                .clone()
                .into_iter()
                .fold(params, |mut params, (k, v)| {
                    params.add_field(k.clone(), juniper_value_from_serde_value(&v));
                    params
                }),
        ),
    );
    juniper::Value::object(jobj)
}

fn juniper_value_from_validation_error_kind(error: &ValidationErrorsKind) -> juniper::Value {
    match error {
        ValidationErrorsKind::Field(xs) => juniper::Value::list(
            xs.into_iter()
                .map(|e| juniper_value_from_validation_error(e))
                .collect(),
        ),
        ValidationErrorsKind::List(map) => {
            let jobj = juniper::Object::with_capacity(map.len());
            let jobj = map.iter().fold(jobj, |mut jobj, (k, v)| {
                jobj.add_field(
                    k.to_string(),
                    ValidationErrorsWrapper(v.as_ref().clone()).into_juniper_value(),
                );
                jobj
            });
            juniper::Value::object(jobj)
        }
        ValidationErrorsKind::Struct(ref x) => {
            ValidationErrorsWrapper(x.as_ref().clone()).into_juniper_value()
        }
    }
}

impl ValidationErrorsWrapper {
    fn into_juniper_value(self) -> juniper::Value {
        let errors_map = self.0.errors();
        let v_errors = juniper::Object::with_capacity(errors_map.len());
        let v_errors = errors_map
            .into_iter()
            .fold(v_errors, |mut v_errors, (k, v)| {
                v_errors.add_field(k.clone(), juniper_value_from_validation_error_kind(v));
                v_errors
            });
        let mut errors = juniper::Object::with_capacity(1);
        errors.add_field("errors", juniper::Value::object(v_errors));
        juniper::Value::object(errors)
    }
}

fn field_error_from_validation_errors(errors: ValidationErrors) -> juniper::FieldError {
    let errors_map = errors.errors();
    let v_errors = juniper::Object::with_capacity(errors_map.len());
    let v_errors = errors_map
        .into_iter()
        .fold(v_errors, |mut v_errors, (k, v)| {
            v_errors.add_field(k.clone(), juniper_value_from_validation_error_kind(v));
            v_errors
        });
    let mut errors = juniper::Object::with_capacity(1);
    errors.add_field("errors", juniper::Value::object(v_errors));
    juniper::FieldError::new("Validation error", juniper::Value::object(errors))
}

impl FromValidationErrors for juniper::FieldError {
    fn from_validation_errors(errors: ValidationErrors) -> Self {
        field_error_from_validation_errors(errors)
    }
}

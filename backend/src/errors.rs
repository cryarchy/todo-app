use juniper::{DefaultScalarValue, Object, Value};
use validator::{ValidationErrors, ValidationErrorsKind};

pub struct ValidationErrorsWrapper(pub ValidationErrors);

impl juniper::IntoFieldError for ValidationErrorsWrapper {
    fn into_field_error(self) -> juniper::FieldError {
        let mut field_errors_map = self.0.into_errors();
        let field_errors_count = field_errors_map.len();
        let errors: Object<DefaultScalarValue> = field_errors_map.iter_mut().fold(
            Object::with_capacity(field_errors_count),
            |mut jobject, (k, v)| {
                let field_errors = match v {
                    ValidationErrorsKind::Field(xs) => {
                        let no_of_errors = xs.len();
                        let field_errors: Object<DefaultScalarValue> = xs.iter_mut().fold(
                            Object::with_capacity(no_of_errors),
                            |mut jobject, error| {
                                jobject
                                    .add_field("code", Value::scalar(error.code.as_ref().clone()));
                                jobject.add_field(
                                    "message",
                                    Value::scalar(
                                        error
                                            .message
                                            .as_mut()
                                            .map(|x| x.clone().into_owned())
                                            .unwrap_or("".to_owned()),
                                    ),
                                );
                                let no_of_params = error.params.len();
                                let params: Object<DefaultScalarValue> =
                                    error.params.iter_mut().fold(
                                        Object::with_capacity(no_of_params),
                                        |mut jobject, (k, v)| {
                                            jobject.add_field(
                                        k.clone(),
                                        match v {
                                            serde_json::value::Value::Null => Value::Null,
                                            serde_json::value::Value::Bool(x) => Value::scalar(*x),
                                            serde_json::value::Value::Number(x) if x.is_f64() => {
                                                Value::scalar(x.as_f64().unwrap_or(0.0) as f64)
                                            }
                                            serde_json::value::Value::Number(x) => {
                                                Value::scalar(x.as_i64().unwrap_or(0) as i32)
                                            }
                                            serde_json::value::Value::String(x) => {
                                                Value::scalar(x.to_owned())
                                            }
                                            _ => Value::scalar("[object]"),
                                        },
                                    );
                                            jobject
                                        },
                                    );
                                jobject.add_field("params", Value::object(params));
                                jobject
                            },
                        );
                        field_errors
                    }
                    _ => Object::with_capacity(0),
                };
                jobject.add_field(k.to_owned(), Value::object(field_errors));
                jobject
            },
        );
        let mut errors_wrapper = Object::with_capacity(1);
        errors_wrapper.add_field("errors", Value::object(errors));
        juniper::FieldError::new("Validation error", Value::object(errors_wrapper))
    }
}

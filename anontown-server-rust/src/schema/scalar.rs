use chrono::{DateTime, Utc};
use juniper::{GraphQLScalar, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeScalar(pub DateTime<Utc>);

#[juniper::graphql_scalar(name = "DateTime")]
impl GraphQLScalar for DateTimeScalar {
    fn resolve(&self) -> Value {
        Value::string(self.0.to_rfc3339())
    }

    fn from_input_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => DateTime::parse_from_rfc3339(s)
                .ok()
                .map(|dt| Self(dt.with_timezone(&Utc))),
            _ => None,
        }
    }
} 
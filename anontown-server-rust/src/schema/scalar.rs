use chrono::{DateTime, Utc};
use juniper::{GraphQLScalar, ScalarValue, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DateTimeScalar(pub DateTime<Utc>);

impl GraphQLScalar for DateTimeScalar {
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_rfc3339())
    }

    fn from_str(value: &str) -> Result<Self, String> {
        DateTime::parse_from_rfc3339(value)
            .map(|dt| DateTimeScalar(dt.with_timezone(&Utc)))
            .map_err(|e| e.to_string())
    }
}

impl From<DateTime<Utc>> for DateTimeScalar {
    fn from(dt: DateTime<Utc>) -> Self {
        DateTimeScalar(dt)
    }
} 
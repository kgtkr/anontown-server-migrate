use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DateQuery {
    pub date: DateTime<Utc>,
    pub kind: DateQueryKind,
}

#[derive(Debug, Clone)]
pub enum DateQueryKind {
    Gt,
    Gte,
    Lt,
    Lte,
} 
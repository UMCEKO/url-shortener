use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize)]
pub struct BodyBuilder {
    message: &'static str,
}

#[derive(Serialize)]
pub struct DataResponseBuilder<T>
where
    T: Serialize,
{
    message: &'static str,
    data: T,
}

impl BodyBuilder {
    pub fn new(message: &'static str) -> BodyBuilder {
        BodyBuilder {
            message
        }
    }

    pub fn add_data<T>(self, data: T) -> DataResponseBuilder<T>
    where
        T: Serialize,
    {
        DataResponseBuilder {
            message: self.message,
            data,
        }
    }
}

impl Display for BodyBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
} 
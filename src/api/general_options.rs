use serde::{Deserialize, Serialize};

pub struct GeneralOptions {
    pub limit: u8,
    pub offset: u32,
}

impl Default for GeneralOptions {
    fn default() -> Self {
        Self {
            limit: 25,
            offset: 0,
        }
    }
}

#[derive(Default)]
pub struct GeneralOptionsBuilder {
    limit: Option<u8>,
    offset: Option<u32>,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum GeneralOptionsBuilderError {
    #[error("{limit:?} is invalid for search, must fall in range 1..=100")]
    InvalidLimit { limit: u8 },
}

impl GeneralOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.offset = Some(offset);

        self
    }

    pub fn build(&self) -> Result<GeneralOptions, GeneralOptionsBuilderError> {
        let limit = self.limit.unwrap_or(25);

        if !(0..=100).contains(&limit) {
            return Err(GeneralOptionsBuilderError::InvalidLimit { limit });
        }

        Ok(GeneralOptions {
            limit,
            offset: self.offset.unwrap_or(0),
        })
    }
}

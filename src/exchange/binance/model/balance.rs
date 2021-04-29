use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_to_decimal")]
    pub free: Decimal,
    #[serde(with = "string_to_decimal")]
    pub locked: Decimal,
}
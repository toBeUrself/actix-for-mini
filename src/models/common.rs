use actix_web::http::StatusCode;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use mysql::{prelude::*, FromValueError, Value};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{glasses::Glasse, shop::Shop};

pub trait Pagination {
    fn get_page(&self) -> u32;
    fn get_size(&self) -> u32;
    fn get_total(&self) -> u32;
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(ApiResultWithShop = ApiResult<Vec<Shop>>, ApiResultWithGlasses = ApiResult<Vec<Glasse>>)]
pub struct ApiResult<T> {
    pub code: u16,
    pub data: T,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CustomTimestamp(i64);

impl TryFrom<Value> for CustomTimestamp {
    type Error = FromValueError;

    fn try_from(value: Value) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self::from_value(value))
    }
}

impl Into<NaiveDateTime> for CustomTimestamp {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0, 1000)
    }
}

impl Into<Value> for CustomTimestamp {
    fn into(self) -> Value {
        self.into()
    }
}

impl FromValue for CustomTimestamp {
    type Intermediate = CustomTimestamp;

    fn from_value(v: Value) -> Self {
        match v {
            Value::Date(year, month, day, hour, min, sec, ms) => {
                let res = NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
                    .unwrap()
                    .and_hms_milli_opt(hour.into(), min.into(), sec.into(), ms);

                match res {
                    Some(dt) => Self(dt.and_utc().timestamp_millis()),
                    _ => Self(0),
                }
            }
            _ => Self(0),
        }
    }
}

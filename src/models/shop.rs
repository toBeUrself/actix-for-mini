use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use mysql::*;
use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize, Serializer};
use utoipa::{IntoParams, ToSchema};

// use actix_web::cookie::time::PrimitiveDateTime; 好像可以从Value::Date转化
use crate::traits::number::SqlEnum;

use super::common::CustomTimestamp;

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ShopListForm {
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Shop {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub telephone: Option<u64>,
    pub status: Option<ShopStatus>,
    pub active: Option<ShopActive>,
    pub create_time: CustomTimestamp,
    pub update_time: CustomTimestamp,
    pub creator: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShopStatus {
    InOperation,
    Cancelled,
    Bankruptly,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShopActive {
    InContract,
    UnContract,
    Cancelled,
    InDisCussion,
    Other,
}
// active: 1: '签约中', 2: '未签约', 3: '已解约', 4: '洽谈中', 99: '其他'
// status: 1: '经营中', 2: '已注销', 3: '已破产', 99: '其他'

impl TryFrom<Value> for ShopActive {
    type Error = FromValueError;

    fn try_from(value: Value) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self::from_value(value))
    }
}

impl FromValue for ShopActive {
    type Intermediate = ShopActive;

    fn from_value(v: Value) -> Self {
        match u32::from_value(v) {
            1 => Self::InContract,
            2 => Self::UnContract,
            3 => Self::Cancelled,
            4 => Self::InDisCussion,
            _ => Self::Other,
        }
    }
}

impl Into<Value> for ShopActive {
    fn into(self) -> Value {
        match self {
            ShopActive::InContract => Value::Bytes(SqlEnum::from(1).into()),
            ShopActive::UnContract => Value::Bytes(SqlEnum::from(2).into()),
            ShopActive::Cancelled => Value::Bytes(SqlEnum::from(3).into()),
            ShopActive::InDisCussion => Value::Bytes(SqlEnum::from(4).into()),
            ShopActive::Other => Value::Bytes(SqlEnum::from(99).into()),
        }
    }
}

impl Into<Value> for ShopStatus {
    fn into(self) -> Value {
        match self {
            ShopStatus::InOperation => Value::Bytes(SqlEnum::from(1).into()),
            ShopStatus::Cancelled => Value::Bytes(SqlEnum::from(2).into()),
            ShopStatus::Bankruptly => Value::Bytes(SqlEnum::from(3).into()),
            ShopStatus::Other => Value::Bytes(SqlEnum::from(99).into()),
        }
    }
}

impl TryFrom<Value> for ShopStatus {
    type Error = FromValueError;

    fn try_from(value: Value) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self::from_value(value))
    }
}

impl FromValue for ShopStatus {
    type Intermediate = ShopStatus;

    fn from_value(v: Value) -> Self {
        match u32::from_value(v) {
            1 => Self::InOperation,
            2 => Self::Cancelled,
            3 => Self::Bankruptly,
            _ => Self::Other,
        }
    }
}

fn serializeDateTime<S>(x: Value, s: S) -> Result<NaiveDateTime, S::Error>
where
    S: Serializer,
{
    let date = NaiveDate::from_ymd_opt(2016, 7, 8).unwrap();
    let time = NaiveTime::from_hms_opt(9, 10, 11).unwrap();

    let dt = NaiveDateTime::new(date, time);

    Ok(dt)
}

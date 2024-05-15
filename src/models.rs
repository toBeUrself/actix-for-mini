use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use mysql::*;
use mysql::{params, prelude::*};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: u16,
    pub data: T,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub telephone: Option<u64>,
    pub status: ShopStatus,
    pub active: ShopActive,
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
            1 => {
                Self::InContract
            },
            2 => Self::UnContract,
            3 => Self::Cancelled,
            4 => Self::InDisCussion,
            _ => {
                Self::Other
            },
        }
    }
}

pub fn get_bytes_from_u32(num: u32) -> Vec<u8> {
    num.to_le_bytes().to_vec()
}

impl Into<Value> for ShopActive {
    fn into(self) -> Value {
        match self {
            ShopActive::InContract => Value::Bytes(get_bytes_from_u32(1)),
            ShopActive::UnContract => Value::Bytes(get_bytes_from_u32(2)),
            ShopActive::Cancelled => Value::Bytes(get_bytes_from_u32(3)),
            ShopActive::InDisCussion => Value::Bytes(get_bytes_from_u32(4)),
            ShopActive::Other => Value::Bytes(get_bytes_from_u32(99)),
        }
    }
}

impl Into<Value> for ShopStatus {
    fn into(self) -> Value {
        match self {
            ShopStatus::InOperation => Value::Bytes(get_bytes_from_u32(1)),
            ShopStatus::Cancelled => Value::Bytes(get_bytes_from_u32(2)),
            ShopStatus::Bankruptly => Value::Bytes(get_bytes_from_u32(3)),
            ShopStatus::Other => Value::Bytes(get_bytes_from_u32(99)),
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
            1 => {
                Self::InOperation
            },
            2 => Self::Cancelled,
            3 => Self::Bankruptly,
            _ => {
                Self::Other
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

        match String::from_value(v) {
            date => {
                let res = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S");

                match res {
                    Ok(dt) => Self(dt.and_utc().timestamp_millis()),
                    Err(_) => Self(0),
                }
            }
            _ => Self(0),
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

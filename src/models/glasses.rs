use serde::{Deserialize, Serialize};

use super::common::CustomTimestamp;

#[derive(Deserialize)]
pub struct GlassesListForm {
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Glasse {
    pub id: u64,
    pub name: String,
    pub email: Option<String>,
    pub r#type: Option<String>, // type是rust保留字段，要用r#注释
    pub style: Option<String>,
    pub descriptoin: Option<String>,
    pub img_url: Option<String>,
    pub telephone: Option<u64>,
    pub create_time: Option<CustomTimestamp>,
    pub update_time: Option<CustomTimestamp>,
    pub creator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlasseInsert {
    pub name: String,
    pub email: Option<String>,
    pub r#type: Option<String>, // type是rust保留字段，要用r#注释
    pub style: Option<String>,
    pub descriptoin: Option<String>,
    pub img_url: Option<String>,
    pub telephone: Option<u64>,
    pub create_time: Option<CustomTimestamp>,
    pub update_time: Option<CustomTimestamp>,
    pub creator: Option<String>,
}

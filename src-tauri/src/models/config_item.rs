use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    pub id: i32,
    pub name: String, // 配置项名称
    pub data: String, // 存储的 JSON 字符串
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperConfigItem {
    pub name: String, // 配置项名称
    pub data: String, // 存储的 JSON 字符串
}

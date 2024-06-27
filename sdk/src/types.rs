use std::any::Any;

use serde::Deserialize;

use crate::errors::WeWorkFinanceSDKError;

#[derive(Deserialize, Debug)]
pub struct EncryptedChatMessage {
    pub seq: Option<u64>,
    #[serde(rename = "msgid")]
    pub msg_id: Option<String>,
    pub publickey_ver: Option<u32>,
    pub encrypt_random_key: Option<String>,
    pub encrypt_chat_msg: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetEncryptedChatMessageListResponse {
    #[serde(flatten)]
    pub error: WeWorkFinanceSDKError,
    #[serde(rename = "chatdata")]
    pub list: Option<Vec<EncryptedChatMessage>>,
}

// 解密后的数据
#[derive(Deserialize, Debug)]
pub struct BaseMessage {
    #[serde(rename = "msgid")]
    pub msg_id: Option<String>,
    pub action: Option<String>,
    pub from: Option<String>,
    pub to_list: Option<Vec<String>>,
    #[serde(rename = "roomid")]
    pub room_id: Option<String>,
    #[serde(rename = "msgtime")]
    pub msg_time: Option<u64>,
    #[serde(rename = "msgtype")]
    pub msg_type: Option<String>,
    #[serde(skip)]
    pub orgin_content: Option<Vec<u8>>,
}

pub struct ChatMessage {
    pub seq: Option<u64>,
    pub publickey_ver: Option<u32>,
    pub content: dyn Any,
}

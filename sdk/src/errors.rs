use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::{ffi::NulError, fmt};

pub(crate) enum WeWorkFinanceSDKKnownCode {
    ParamsError = 10000,
    NetworkError = 10001,
    ParseError = 10002,
    SystemError = 10003,
    SecretError = 10004,
    FileIdError = 10005,
    DecryptError = 10006,
    SecretMissError = 10007,
    EncryptKeyError = 10008,
    IPNotWhiteListError = 10009,
    DataExpiredError = 10010,
    TokenExpiredError = 10011,
}

impl WeWorkFinanceSDKKnownCode {
    fn message(&self) -> &'static str {
        match self {
            WeWorkFinanceSDKKnownCode::ParamsError => "参数错误，请求参数错误",
            WeWorkFinanceSDKKnownCode::NetworkError => "网络错误，网络请求错误",
            WeWorkFinanceSDKKnownCode::ParseError => "数据解析失败",
            WeWorkFinanceSDKKnownCode::SystemError => "系统失败",
            WeWorkFinanceSDKKnownCode::SecretError => "密钥错误导致加密失败",
            WeWorkFinanceSDKKnownCode::FileIdError => "fileid错误",
            WeWorkFinanceSDKKnownCode::DecryptError => "解密失败",
            WeWorkFinanceSDKKnownCode::SecretMissError => {
                "找不到消息加密版本的私钥，需要重新传入私钥对"
            }
            WeWorkFinanceSDKKnownCode::EncryptKeyError => "解析encrypt_key出错",
            WeWorkFinanceSDKKnownCode::IPNotWhiteListError => "ip非法",
            WeWorkFinanceSDKKnownCode::DataExpiredError => "数据过期",
            WeWorkFinanceSDKKnownCode::TokenExpiredError => "证书过期",
        }
    }
}

impl From<WeWorkFinanceSDKKnownCode> for WeWorkFinanceSDKError {
    fn from(error: WeWorkFinanceSDKKnownCode) -> Self {
        WeWorkFinanceSDKError {
            err_msg: error.message().to_string(),
            err_code: error as i32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct WeWorkFinanceSDKError {
    #[serde(rename = "errcode")]
    pub err_code: i32,
    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

impl WeWorkFinanceSDKError {
    pub fn new(code: i32) -> Self {
        let msg = match code {
            10000 => WeWorkFinanceSDKKnownCode::ParamsError.message(),
            10001 => WeWorkFinanceSDKKnownCode::NetworkError.message(),
            10002 => WeWorkFinanceSDKKnownCode::ParseError.message(),
            10003 => WeWorkFinanceSDKKnownCode::SystemError.message(),
            10004 => WeWorkFinanceSDKKnownCode::SecretError.message(),
            10005 => WeWorkFinanceSDKKnownCode::FileIdError.message(),
            10006 => WeWorkFinanceSDKKnownCode::DecryptError.message(),
            10007 => WeWorkFinanceSDKKnownCode::SecretMissError.message(),
            10008 => WeWorkFinanceSDKKnownCode::EncryptKeyError.message(),
            10009 => WeWorkFinanceSDKKnownCode::IPNotWhiteListError.message(),
            10010 => WeWorkFinanceSDKKnownCode::DataExpiredError.message(),
            10011 => WeWorkFinanceSDKKnownCode::TokenExpiredError.message(),
            _ => "未知错误",
        };
        WeWorkFinanceSDKError {
            err_code: code,
            err_msg: msg.to_string(),
        }
    }
}

impl std::error::Error for WeWorkFinanceSDKError {}

impl fmt::Display for WeWorkFinanceSDKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WeWorkFinanceSDKError {}: {}",
            self.err_code, self.err_msg
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SDKError {
    #[error("NulError: {0}")]
    NulError(#[from] NulError),
    #[error("WeWorkFinanceSDKError: {0}")]
    WeWorkFinanceSDKError(#[from] WeWorkFinanceSDKError),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] SerdeError),
}

impl From<WeWorkFinanceSDKKnownCode> for SDKError {
    fn from(error: WeWorkFinanceSDKKnownCode) -> Self {
        SDKError::WeWorkFinanceSDKError(error.into())
    }
}

pub(crate) trait WeWorkFinanceErrorExt {
    fn to_wework_finance_error(&self) -> WeWorkFinanceSDKError;
}

impl WeWorkFinanceErrorExt for i32 {
    fn to_wework_finance_error(&self) -> WeWorkFinanceSDKError {
        WeWorkFinanceSDKError::new(*self)
    }
}

pub(crate) trait SDKErrorExt {
    fn to_sdk_error(&self) -> SDKError;
}

impl SDKErrorExt for i32 {
    fn to_sdk_error(&self) -> SDKError {
        SDKError::WeWorkFinanceSDKError(WeWorkFinanceSDKError::new(*self))
    }
}

use serde_json;
use std::ffi::CString;
use std::os::raw::{c_int, c_uint, c_ulonglong};

use crate::bindings::{
    DestroySdk, GetChatData, GetContentFromSlice, GetSliceLen, Init, NewSdk, NewSlice, Slice_t,
    WeWorkFinanceSdk_t,
};
use crate::errors::{SDKError, SDKErrorExt, WeWorkFinanceSDKError, WeWorkFinanceSDKKnownCode};
use crate::types::{BaseMessage, EncryptedChatMessage, GetEncryptedChatMessageListResponse};

pub struct Client {
    sdk: *mut WeWorkFinanceSdk_t,
    private_key: String,
}

pub fn new(corp_id: String, corp_secret: String, private_key: String) -> Result<Client, SDKError> {
    let sdk = unsafe { NewSdk() };
    if sdk.is_null() {
        return Err(WeWorkFinanceSDKKnownCode::SystemError.into());
    }
    let corp_id = CString::new(corp_id)?;
    let corp_secret: CString = CString::new(corp_secret)?;

    unsafe {
        let ret = Init(sdk, corp_id.as_ptr(), corp_secret.as_ptr());
        if ret != 0 {
            return Err(ret.to_sdk_error());
        }
    }

    Ok(Client { sdk, private_key })
}

impl Client {
    pub fn free(&mut self) {
        if self.sdk.is_null() {
            return;
        }
        unsafe {
            DestroySdk(self.sdk);
            self.sdk = std::ptr::null_mut();
        }
    }

    pub fn get_chat_data(
        &self,
        seq: u64,
        limit: u64,
        proxy: String,
        passwd: String,
        timeout: i32,
    ) -> Result<Vec<EncryptedChatMessage>, SDKError> {
        let proxy = CString::new(proxy)?;
        let passwd = CString::new(passwd)?;
        let slice = unsafe { NewSlice() };
        if self.sdk.is_null() {
            return Err(WeWorkFinanceSDKKnownCode::ParamsError.into());
        }
        let ret = unsafe {
            GetChatData(
                self.sdk,
                seq as c_ulonglong,
                limit as c_uint,
                proxy.as_ptr(),
                passwd.as_ptr(),
                timeout as c_int,
                slice,
            )
        };
        if ret != 0 {
            return Err(ret.to_sdk_error());
        }
        let buf = self.get_content_from_slice(slice);
        let resp: GetEncryptedChatMessageListResponse = serde_json::from_slice(&buf)?;
        if resp.error.err_code != 0 {
            return Err(WeWorkFinanceSDKError::new(resp.error.err_code).into());
        }
        Ok(resp.list.unwrap_or_default())
    }

    pub fn decrypt_data(
        &self,
        encrypt_random_key: String,
        encrypt_chat_msg: String,
    ) -> Result<BaseMessage, SDKError> {
        todo!()
    }

    fn get_content_from_slice(&self, slice: *mut Slice_t) -> Vec<u8> {
        unsafe {
            let data_ptr = GetContentFromSlice(slice) as *const u8;
            let len = GetSliceLen(slice) as usize;
            let data_slice = std::slice::from_raw_parts(data_ptr, len);
            Vec::from(data_slice)
        }
    }
}

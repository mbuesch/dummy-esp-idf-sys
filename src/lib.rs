// -*- coding: utf-8 -*-

use std::ffi::{c_char, c_int};

#[allow(non_camel_case_types)]
pub type esp_err_t = c_int;

#[derive(Debug)]
pub struct EspError;

impl EspError {
    pub fn convert(_error: esp_err_t) -> Result<(), Self> {
        Ok(())
    }

    pub const fn from(_error: esp_err_t) -> Option<Self> {
        Some(EspError)
    }
}

impl std::error::Error for EspError {}

impl std::fmt::Display for EspError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub unsafe fn esp_wifi_set_country_code(
    _country: *const c_char,
    _ieee80211d_enabled: bool,
) -> esp_err_t {
    0
}

pub unsafe fn esp_wifi_get_country_code(_country: *mut c_char) -> esp_err_t {
    0
}

#[macro_export]
macro_rules! esp {
    ($err:expr) => {{
        $crate::EspError::convert($err as $crate::esp_err_t)
    }};
}

// vim: ts=4 sw=4 expandtab

// -*- coding: utf-8 -*-

#[derive(Debug)]
pub struct EspError;

impl std::error::Error for EspError {
}

impl std::fmt::Display for EspError {
     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
     }
}

// vim: ts=4 sw=4 expandtab

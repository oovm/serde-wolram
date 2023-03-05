mod errors;

use std::io::{Read, Write};
pub use crate::errors::{ZapError, ZapResult};


pub struct Zap {}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CompressionMethod {
    Nothing,
}

/// A block of data which is a multiple of 4k
pub struct ZapBlock {
    pub metadata: Metadata,
    pub start: u64,
    pub end: u64,
}

pub struct Metadata {
    pub compression_method: CompressionMethod,
}

impl Zap {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read<S: Read>(&self, path: S) -> ZapResult<Vec<u8>> {
        Ok(vec![])
    }

    pub fn open<S: Read + Write>(&self, stream: S) -> ZapResult<()> {
        Ok(())
    }

    pub fn compress(&self, data: &[u8], method: CompressionMethod) -> ZapResult<Vec<u8>> {
        match method {
            CompressionMethod::Nothing => Ok(data.to_vec()),
        }
    }
}

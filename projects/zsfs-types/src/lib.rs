use std::io::{Read, Write};
use std::path::Path;

pub use crate::errors::{ZapError, ZapResult};

mod errors;
mod meta_block;
pub use crate::meta_block::SuperBlock;

pub struct Zap {}

pub struct ZfsMount {
    meta: SuperBlock,
}

impl ZfsMount {
    /// a.number.zs/zfs/zsfs
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {

        }
    }
}



/// A block of data which is a multiple of 4k
pub struct ZapBlock {
    pub metadata: Metadata,
    pub created_at: u64,
    pub modified_at: u64,
    pub direct_blocks: [u32; DIRECT_POINTERS as usize],
    pub indirect_block: u32,
    pub double_indirect_block: u32,
    pub crc32: u32,
}

pub struct Metadata {
    pub compression_level: u8,
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

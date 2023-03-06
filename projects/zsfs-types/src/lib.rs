use std::io::{Read, Write};

pub use crate::errors::{ZapError, ZapResult};

mod errors;

pub struct Zap {}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CompressionMethod {
    Nothing,
}

pub struct SuperBlock {
    pub magic: [u8; 4],
    pub block_size: u32,
    pub modified_at: Option<u64>,
    pub last_mounted_at: Option<u64>,
    pub block_count: u32,
    pub inode_count: u32,
    pub free_blocks: u32,
    pub free_inodes: u32,
    pub groups: u32,
    pub data_blocks_per_group: u32,
    pub uid: u32,
    pub gid: u32,
    pub checksum: u32,
}

/// A block of data which is a multiple of 4k
pub struct ZapBlock {
    pub metadata: Metadata,
    pub created_at: u64,
    pub modified_at: u64,
    pub direct_blocks: [u32; DIRECT_POINTERS as usize],
    pub indirect_block: u32,
    pub double_indirect_block: u32,
    pub checksum: u32,
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

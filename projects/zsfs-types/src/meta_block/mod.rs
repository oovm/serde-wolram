#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuperBlock {
    magic: [u8; 4],
    version: u32,
    meta: SuperBlockVersioned,
}

pub enum SuperBlockVersioned {
    Version0(SuperBlockV0)
}

pub struct SuperBlockV0 {
    pub block_size: u32,
    /// 0 means do not use volume compression
    pub volume_size: u32,
    pub default_compression_level: u8,
    pub block_count: u32,
    pub inode_count: u32,
    pub free_blocks: u32,
    pub free_inodes: u32,
    pub groups: u32,
    pub data_blocks_per_group: u32,
    pub crc32: u32,
}

impl SuperBlock {
    pub fn load_version(version: u32) {}
}

impl Default for SuperBlock {
    fn default() -> Self {
        Self {
            magic: *b"zsfs",
            version: 1,
            block_size: 0,
            volume_size: 0,
            default_compression_level: 0,
            block_count: 0,
            inode_count: 0,
            free_blocks: 0,
            free_inodes: 0,
            groups: 0,
            data_blocks_per_group: 0,
            crc32: 0,
        }
    }
}
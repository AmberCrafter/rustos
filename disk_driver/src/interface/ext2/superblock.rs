use super::SUPERBLOCK_SIZE;

/// SuperBlock
// ref: https://www.nongnu.org/ext2-doc/ext2.html#superblock
#[derive(Debug)]
#[repr(C)]
pub struct SuperBlock {
    pub inodes_count: u32,
    pub blocks_count: u32,
    pub r_blocks_count: u32,
    pub free_blocks_count: u32,
    pub free_inodes_count: u32,
    pub first_data_block: u32,
    pub log_block_size: u32,
    pub log_frag_size: u32,
    pub blocks_per_group: u32,
    pub frags_per_group: u32,
    pub inodes_per_group: u32,
    pub mtime: u32,
    pub wtime: u32,
    pub mnt_count: u16,
    pub max_mnt_count: u16,
    pub magic: u16,
    pub state: u16,
    pub errors: u16,
    pub minor_rev_level: u16,
    pub lastcheck: u32,
    pub checkinterval: u32,
    pub creator_os: u32,
    pub rev_level: u32,
    pub def_resuid: u16,
    pub def_resgid: u16,
    pub first_info: u32,
    pub inode_size: u16,
    pub block_group_nr: u16,
    pub feature_compat: u32,
    pub feature_incompat: u32,
    pub feature_ro_compat: u32,
    pub uuid: [u8; 16],
    pub volume_name: [u8; 16],
    pub last_mounted: [u8; 64],
    pub algo_bitmap: u32,
    pub prealloc_blocks: u8,
    pub prealloc_dir_blocks: u8,
    pub alignment: u16,
    pub journal_uuid: [u8; 16],
    pub journal_inum: u32,
    pub journal_dev: u32,
    pub last_orphan: u32,
    pub hash_seed: [u32; 4],
    pub def_hash_version: u8,
    pub padding: [u8; 3],
    pub default_mount_options: u32,
    pub first_meta_bg: u32,
    pub unused: [u8; SUPERBLOCK_SIZE - 264], // 760
}

impl Default for SuperBlock {
    fn default() -> Self {
        Self {
            inodes_count: 0,
            blocks_count: 0,
            r_blocks_count: 0,
            free_blocks_count: 0,
            free_inodes_count: 0,
            first_data_block: 0,
            log_block_size: 0,
            log_frag_size: 0,
            blocks_per_group: 0,
            frags_per_group: 0,
            inodes_per_group: 0,
            mtime: 0,
            wtime: 0,
            mnt_count: 0,
            max_mnt_count: 0,
            magic: 0,
            state: 0,
            errors: 0,
            minor_rev_level: 0,
            lastcheck: 0,
            checkinterval: 0,
            creator_os: 0,
            rev_level: 0,
            def_resuid: 0,
            def_resgid: 0,
            first_info: 0,
            inode_size: 0,
            block_group_nr: 0,
            feature_compat: 0,
            feature_incompat: 0,
            feature_ro_compat: 0,
            uuid: [0; 16],
            volume_name: [0; 16],
            last_mounted: [0; 64],
            algo_bitmap: 0,
            prealloc_blocks: 0,
            prealloc_dir_blocks: 0,
            alignment: 0,
            journal_uuid: [0; 16],
            journal_inum: 0,
            journal_dev: 0,
            last_orphan: 0,
            hash_seed: [0; 4],
            def_hash_version: 0,
            padding: [0; 3],
            default_mount_options: 0,
            first_meta_bg: 0,
            unused: [0; SUPERBLOCK_SIZE - 264],
        }
    }
}

impl SuperBlock {
    pub fn new(
        inodes_count: u32,
        blocks_count: u32,
        r_blocks_count: u32,
        free_blocks_count: u32,
        free_inodes_count: u32,
        first_data_block: u32,
        log_block_size: u32,
        log_frag_size: u32,
        blocks_per_group: u32,
        frags_per_group: u32,
        inodes_per_group: u32,
        mtime: u32,
        wtime: u32,
        mnt_count: u16,
        max_mnt_count: u16,
        magic: u16,
        state: u16,
        errors: u16,
        minor_rev_level: u16,
        lastcheck: u32,
        checkinterval: u32,
        creator_os: u32,
        rev_level: u32,
        def_resuid: u16,
        def_resgid: u16,
        first_info: u32,
        inode_size: u16,
        block_group_nr: u16,
        feature_compat: u32,
        feature_incompat: u32,
        feature_ro_compat: u32,
        uuid: [u8; 16],
        volume_name: [u8; 16],
        last_mounted: [u8; 64],
        algo_bitmap: u32,
        prealloc_blocks: u8,
        prealloc_dir_blocks: u8,
        alignment: u16,
        journal_uuid: [u8; 16],
        journal_inum: u32,
        journal_dev: u32,
        last_orphan: u32,
        hash_seed: [u32; 4],
        def_hash_version: u8,
        padding: [u8; 3],
        default_mount_options: u32,
        first_meta_bg: u32,
        unused: [u8; SUPERBLOCK_SIZE - 264],
    ) -> Self {
        Self {
            inodes_count,
            blocks_count,
            r_blocks_count,
            free_blocks_count,
            free_inodes_count,
            first_data_block,
            log_block_size,
            log_frag_size,
            blocks_per_group,
            frags_per_group,
            inodes_per_group,
            mtime,
            wtime,
            mnt_count,
            max_mnt_count,
            magic,
            state,
            errors,
            minor_rev_level,
            lastcheck,
            checkinterval,
            creator_os,
            rev_level,
            def_resuid,
            def_resgid,
            first_info,
            inode_size,
            block_group_nr,
            feature_compat,
            feature_incompat,
            feature_ro_compat,
            uuid,
            volume_name,
            last_mounted,
            algo_bitmap,
            prealloc_blocks,
            prealloc_dir_blocks,
            alignment,
            journal_uuid,
            journal_inum,
            journal_dev,
            last_orphan,
            hash_seed,
            def_hash_version,
            padding,
            default_mount_options,
            first_meta_bg,
            unused,
        }
    }
}
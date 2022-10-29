// use for PIO mode
pub struct ATAdisk {
    io_port: IoRegistor,
    ctl_port: ControlRegistor
}

#[repr(C)]
pub struct IoRegistor {
    data: u16,
    error: u16,             // u8/u16
    features: u16,          // u8/u16
    sector_count: u16,      // u8/u16
    lba_lo: u16,            // u8/u16
    lab_mid: u16,           // u8/u16
    lab_hi: u16,            // u8/u16
    lab_ext: u8,            // low-half of base+6 registor (only used on LBA28 mode)
    config: u8,             // high-half of base+6 registor
    status: u8,
    command: u8,
}

#[repr(C)]
pub struct ControlRegistor {
    alternate_status: u8,
    device_control: u8,
    driver_address: u8,
}

pub trait IO {
    fn self_check(&self);
    fn read(&self, lba: usize) -> [u8];
    fn write(&mut self, lba: usize, data: &[u8]);
    fn reset(&mut self);
}
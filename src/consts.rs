pub const CARDMON_MIN: u16 = 0x1c9;
pub const CARDMON_MAX: u16 = 0x1d0;
pub const BOSSES: [u16; 26] = [
    0x46, 0x7c, 0x8d, 0xb2, 0x151, 0x164, 0x166, 0x1b4, 0x1ba, 0x1bb, 0x1bc, 0x1bd, 0x1be, 0x1bf,
    0x1c0, 0x1c1, 0x1c2, 0x1c3, 0x1c4, 0x1c5, 0x1c6, 0x1c7, 0x1c8, 0x1d1, 0x1d2, 0x1d3,
];
pub const TRICERAMON_ID: u16 = 0xcb;

pub const STATS_FILE: &str = "./extract/AAA/PRO/SDIGIEDT.PRO";
pub const ENCOUNTERS_FILE: &str = "./extract/AAA/PRO/FIELDSTG.PRO";
pub const SHOPS_FILE: &str = "./extract/AAA/PRO/STITSHOP.PRO";

pub const SHOPS_LEN: usize = 30;

pub const OVERLAYADDRESS: u32 = 0x800100c4;

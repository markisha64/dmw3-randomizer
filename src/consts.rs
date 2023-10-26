pub const CARDMON_MIN: u16 = 0x1c9;
pub const CARDMON_MAX: u16 = 0x1d0;
pub const BOSSES: [u16; 26] = [
    0x46, 0x7c, 0x8d, 0xb2, 0x151, 0x164, 0x166, 0x1b4, 0x1ba, 0x1bb, 0x1bc, 0x1bd, 0x1be, 0x1bf,
    0x1c0, 0x1c1, 0x1c2, 0x1c3, 0x1c4, 0x1c5, 0x1c6, 0x1c7, 0x1c8, 0x1d1, 0x1d2, 0x1d3,
];

pub const TRICERAMON_ID: u16 = 0xcb;
pub const ZANBAMON_ID: u16 = 0x151;
pub const GALACTICMON_1ST_PHASE: u16 = 0x1ba;
pub const GALACTICMON_IDS: [u16; 3] = [0x1ba, 0x1d2, 0x1d3];

pub const STATS_FILE: &str = "AAA/PRO/SDIGIEDT.PRO";
pub const ENCOUNTERS_FILE: &str = "AAA/PRO/FIELDSTG.PRO";
pub const SHOPS_FILE: &str = "AAA/PRO/STITSHOP.PRO";
pub const EXP_FILE: &str = "AAA/PRO/STFGTREP.PRO";

pub const SHOPS_LEN: usize = 30;

pub const OVERLAYADDRESS: u32 = 0x800100c4;

// min is 0, max is how much space there is for shops
pub const MIN_SHOP_ITEMS: i64 = 0;
pub const MAX_SHOP_ITEMS: i64 = 37;

pub const MIN_STAT_RANGE: i64 = 0;
pub const MAX_STAT_RANGE: i64 = 150;

// don't want free items
pub const MIN_SELL_PRICE: i64 = 1;
// max u16 / 2
pub const MAX_SELL_PRICE: i64 = 32768;

pub const TNT_BALL_ID: u16 = 0x5a;

pub const MULTI_HIT: u8 = 0x9;

pub const CHAMPIONS: [u16; 10] = [
    0x182, 0x103, 0x183, 0x176, 0x14, 0x16f, 0x184, 0x5, 0x104, 0xea,
];

pub const ULTIMATES: [u16; 13] = [
    0x186, 0x187, 0x185, 0xd3, 0xc, 0x1b, 0x177, 0x13, 0x38, 0x170, 0xfe, 0x1a, 0x6,
];

pub const MEGAS: [u16; 13] = [
    0x188, 0x189, 0x18a, 0xd5, 0x94, 0x171, 0x178, 0xd6, 0xc4, 0x90, 0x10b, 0x3b, 0x42,
];

pub const MEGAPLUS: [u16; 5] = [0x96, 0xe6, 0x174, 0x17a, 0x167];

pub const ULTRAS: [u16; 3] = [0x97, 0x179, 0x17d];

pub const ROOKIE_COUNT: usize = 8;
pub const DIGIVOLUTION_COUNT: usize = 44;

pub const MIN_STAT_AFFINITY: u8 = 1;
pub const MAX_STAT_AFFINITY: u8 = 5;

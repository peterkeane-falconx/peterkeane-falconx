#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Venue {
    BINANCE = 0x1_u8, 
    BINANCEFUT = 0x2_u8, 
    OKX = 0x3_u8, 
    DERIBIT = 0x4_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for Venue {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::BINANCE, 
            0x2_u8 => Self::BINANCEFUT, 
            0x3_u8 => Self::OKX, 
            0x4_u8 => Self::DERIBIT, 
            _ => Self::NullVal,
        }
    }
}

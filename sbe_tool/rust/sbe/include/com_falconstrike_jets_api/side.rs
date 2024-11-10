#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Side {
    BUY = 49_u8, 
    SELL = 50_u8, 
    BUY_MINUS = 51_u8, 
    SELL_PLUS = 52_u8, 
    SELL_SHORT = 53_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for Side {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::BUY, 
            50_u8 => Self::SELL, 
            51_u8 => Self::BUY_MINUS, 
            52_u8 => Self::SELL_PLUS, 
            53_u8 => Self::SELL_SHORT, 
            _ => Self::NullVal,
        }
    }
}

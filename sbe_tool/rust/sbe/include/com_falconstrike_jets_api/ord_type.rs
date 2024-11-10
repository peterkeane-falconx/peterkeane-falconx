#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrdType {
    MARKET = 49_u8, 
    LIMIT = 50_u8, 
    STOP = 51_u8, 
    STOP_LIMIT = 52_u8, 
    WITH_OR_WITHOUT = 54_u8, 
    LIMIT_OR_BETTER = 55_u8, 
    LIMIT_WITH_OR_WITHOUT = 56_u8, 
    MARKET_IF_TOUCHED = 74_u8, 
    PEGGED = 80_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for OrdType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::MARKET, 
            50_u8 => Self::LIMIT, 
            51_u8 => Self::STOP, 
            52_u8 => Self::STOP_LIMIT, 
            54_u8 => Self::WITH_OR_WITHOUT, 
            55_u8 => Self::LIMIT_OR_BETTER, 
            56_u8 => Self::LIMIT_WITH_OR_WITHOUT, 
            74_u8 => Self::MARKET_IF_TOUCHED, 
            80_u8 => Self::PEGGED, 
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    DAY = 48_u8, 
    GOOD_TILL_CANCEL = 49_u8, 
    AT_THE_OPENING = 50_u8, 
    IMMEDIATE_OR_CANCEL = 51_u8, 
    FILL_OR_KILL = 52_u8, 
    GOOD_TILL_CROSSING = 53_u8, 
    GOOD_TILL_DATE = 54_u8, 
    AT_THE_CLOSE = 55_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for TimeInForce {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::DAY, 
            49_u8 => Self::GOOD_TILL_CANCEL, 
            50_u8 => Self::AT_THE_OPENING, 
            51_u8 => Self::IMMEDIATE_OR_CANCEL, 
            52_u8 => Self::FILL_OR_KILL, 
            53_u8 => Self::GOOD_TILL_CROSSING, 
            54_u8 => Self::GOOD_TILL_DATE, 
            55_u8 => Self::AT_THE_CLOSE, 
            _ => Self::NullVal,
        }
    }
}

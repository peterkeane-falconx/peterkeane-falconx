#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TechnicalRejectRespondTo {
    NEW_ORDER = 49_u8, 
    CANCEL_REQUEST = 50_u8, 
    CANCEL_REPLACE_REQUEST = 51_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for TechnicalRejectRespondTo {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::NEW_ORDER, 
            50_u8 => Self::CANCEL_REQUEST, 
            51_u8 => Self::CANCEL_REPLACE_REQUEST, 
            _ => Self::NullVal,
        }
    }
}

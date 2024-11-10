#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejReason {
    TOO_LATE_TO_CANCEL = 48_u8, 
    UNKNOWN_ORDER = 49_u8, 
    BROKER_OPTION = 50_u8, 
    ORDER_ALREADY_PENDING_UPDATE = 51_u8, 
    UNABLE_TO_PROCESS_q = 52_u8, 
    ORIG_ORDER_MOD_TIME_INVALID = 53_u8, 
    DUPLICATE_CLORDID_RECEIVED = 54_u8, 
    OTHER = 57_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for CxlRejReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::TOO_LATE_TO_CANCEL, 
            49_u8 => Self::UNKNOWN_ORDER, 
            50_u8 => Self::BROKER_OPTION, 
            51_u8 => Self::ORDER_ALREADY_PENDING_UPDATE, 
            52_u8 => Self::UNABLE_TO_PROCESS_q, 
            53_u8 => Self::ORIG_ORDER_MOD_TIME_INVALID, 
            54_u8 => Self::DUPLICATE_CLORDID_RECEIVED, 
            57_u8 => Self::OTHER, 
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RejectReason {
    RISK_REJECT = 48_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for RejectReason {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            48_u8 => Self::RISK_REJECT, 
            _ => Self::NullVal,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CxlRejResponseTo {
    ORDER_CANCEL_REQUEST = 49_u8, 
    ORDER_CANCEL_REPLACE_REQUEST = 50_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for CxlRejResponseTo {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::ORDER_CANCEL_REQUEST, 
            50_u8 => Self::ORDER_CANCEL_REPLACE_REQUEST, 
            _ => Self::NullVal,
        }
    }
}

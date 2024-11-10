#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InboundMessageOrigin {
    ACCEPTOR_FIX_GW = 49_u8, 
    REST_GW = 50_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for InboundMessageOrigin {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            49_u8 => Self::ACCEPTOR_FIX_GW, 
            50_u8 => Self::REST_GW, 
            _ => Self::NullVal,
        }
    }
}

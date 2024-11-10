#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SubscriptionFlag {
    TopFlag = 0x1_u8, 
    DepthFlag = 0x2_u8, 
    TradeFlag = 0x3_u8, 
    TickerFlag = 0x4_u8, 
    #[default]
    NullVal = 0xff_u8, 
}
impl From<u8> for SubscriptionFlag {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x1_u8 => Self::TopFlag, 
            0x2_u8 => Self::DepthFlag, 
            0x3_u8 => Self::TradeFlag, 
            0x4_u8 => Self::TickerFlag, 
            _ => Self::NullVal,
        }
    }
}

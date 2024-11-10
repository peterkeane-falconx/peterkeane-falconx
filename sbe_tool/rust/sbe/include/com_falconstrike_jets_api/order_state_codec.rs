use crate::*;

pub use encoder::OrderStateEncoder;
pub use decoder::OrderStateDecoder;

pub const ENCODED_LENGTH: usize = 19;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderStateEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for OrderStateEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> OrderStateEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn cum_qty_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn avg_px_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 9;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn status(&mut self, value: ord_status::OrdStatus) {
            let offset = self.offset + 18;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderStateDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for OrderStateDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for OrderStateDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> OrderStateDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn cum_qty_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn avg_px_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 9;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn status(&self) -> ord_status::OrdStatus {
            self.get_buf().get_u8_at(self.offset + 18).into()
        }

    }
} // end decoder mod 

use crate::*;

pub use encoder::CancelReplaceBodyEncoder;
pub use decoder::CancelReplaceBodyDecoder;

pub const ENCODED_LENGTH: usize = 27;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct CancelReplaceBodyEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for CancelReplaceBodyEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> CancelReplaceBodyEncoder<P> where P: Writer<'a> + Default {
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
        pub fn order_qty_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 9;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn stop_px_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 18;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct CancelReplaceBodyDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for CancelReplaceBodyDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for CancelReplaceBodyDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> CancelReplaceBodyDecoder<P> where P: Reader<'a> + Default {
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
        pub fn order_qty_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 9;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn stop_px_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 18;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

    }
} // end decoder mod 

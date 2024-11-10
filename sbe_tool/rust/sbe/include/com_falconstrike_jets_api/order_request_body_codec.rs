use crate::*;

pub use encoder::OrderRequestBodyEncoder;
pub use decoder::OrderRequestBodyDecoder;

pub const ENCODED_LENGTH: usize = 102;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderRequestBodyEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for OrderRequestBodyEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> OrderRequestBodyEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: side::Side) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_type(&mut self, value: ord_type::OrdType) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'symbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 32
        /// - version: 0
        #[inline]
        pub fn symbol(&mut self, value: &[u8; 32]) {
            let offset = self.offset + 2;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_qty_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 34;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: time_in_force::TimeInForce) {
            let offset = self.offset + 43;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 44;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn stop_px_encoder(self) -> decimal_codec::DecimalEncoder<Self> {
            let offset = self.offset + 53;
            decimal_codec::DecimalEncoder::default().wrap(self, offset)
        }

        /// primitive array field 'account'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 62
        /// - encodedLength: 40
        /// - version: 0
        #[inline]
        pub fn account(&mut self, value: &[u8; 40]) {
            let offset = self.offset + 62;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderRequestBodyDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> ActingVersion for OrderRequestBodyDecoder<P> where P: Reader<'a> + ActingVersion + Default {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.parent.as_ref().unwrap().acting_version()
        }
    }

    impl<'a, P> Reader<'a> for OrderRequestBodyDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> OrderRequestBodyDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> side::Side {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_type(&self) -> ord_type::OrdType {
            self.get_buf().get_u8_at(self.offset + 1).into()
        }

        #[inline]
        pub fn symbol(&self) -> [u8; 32] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 2)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_qty_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 34;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> time_in_force::TimeInForce {
            self.get_buf().get_u8_at(self.offset + 43).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 44;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn stop_px_decoder(self) -> decimal_codec::DecimalDecoder<Self> {
            let offset = self.offset + 53;
            decimal_codec::DecimalDecoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn account(&self) -> [u8; 40] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 62)
        }

    }
} // end decoder mod 

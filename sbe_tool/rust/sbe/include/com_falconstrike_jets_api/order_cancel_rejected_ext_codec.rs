use crate::*;

pub use encoder::OrderCancelRejectedExtEncoder;
pub use decoder::OrderCancelRejectedExtDecoder;

pub const SBE_BLOCK_LENGTH: u16 = 148;
pub const SBE_TEMPLATE_ID: u16 = 2007;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "0.1.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct OrderCancelRejectedExtEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderCancelRejectedExtEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderCancelRejectedExtEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCancelRejectedExtEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive field 'sessionId'
        /// - min value: -32767
        /// - max value: 32767
        /// - null value: -32768
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 2
        /// - version: 0
        #[inline]
        pub fn session_id(&mut self, value: i16) {
            let offset = self.offset;
            self.get_buf_mut().put_i16_at(offset, value);
        }

        /// primitive array field 'clOrdId'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 40
        /// - version: 0
        #[inline]
        pub fn cl_ord_id(&mut self, value: &[u8; 40]) {
            let offset = self.offset + 2;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive array field 'origClOrdId'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 42
        /// - encodedLength: 40
        /// - version: 0
        #[inline]
        pub fn orig_cl_ord_id(&mut self, value: &[u8; 40]) {
            let offset = self.offset + 42;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

        /// primitive field 'orderId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 82
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_id(&mut self, value: i64) {
            let offset = self.offset + 82;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_response_to(&mut self, value: cxl_rej_response_to::CxlRejResponseTo) {
            let offset = self.offset + 90;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_reason(&mut self, value: cxl_rej_reason::CxlRejReason) {
            let offset = self.offset + 91;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'transactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 92
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn transact_time(&mut self, value: i64) {
            let offset = self.offset + 92;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'sendingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 100
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn sending_time(&mut self, value: i64) {
            let offset = self.offset + 100;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive array field 'account'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ASCII
        /// - semanticType: null
        /// - encodedOffset: 108
        /// - encodedLength: 40
        /// - version: 0
        #[inline]
        pub fn account(&mut self, value: &[u8; 40]) {
            let offset = self.offset + 108;
            let buf = self.get_buf_mut();
            buf.put_bytes_at(offset, value);
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderCancelRejectedExtDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for OrderCancelRejectedExtDecoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for OrderCancelRejectedExtDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderCancelRejectedExtDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCancelRejectedExtDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn session_id(&self) -> i16 {
            self.get_buf().get_i16_at(self.offset)
        }

        #[inline]
        pub fn cl_ord_id(&self) -> [u8; 40] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 2)
        }

        #[inline]
        pub fn orig_cl_ord_id(&self) -> [u8; 40] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 42)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 82)
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_response_to(&self) -> cxl_rej_response_to::CxlRejResponseTo {
            self.get_buf().get_u8_at(self.offset + 90).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_reason(&self) -> cxl_rej_reason::CxlRejReason {
            self.get_buf().get_u8_at(self.offset + 91).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 92)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn sending_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 100)
        }

        #[inline]
        pub fn account(&self) -> [u8; 40] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 108)
        }

    }

} // end decoder


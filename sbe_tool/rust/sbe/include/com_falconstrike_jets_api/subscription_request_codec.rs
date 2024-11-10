use crate::*;

pub use encoder::SubscriptionRequestEncoder;
pub use decoder::SubscriptionRequestDecoder;

pub const SBE_BLOCK_LENGTH: u16 = 26;
pub const SBE_TEMPLATE_ID: u16 = 1004;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "0.1.0";

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct SubscriptionRequestEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for SubscriptionRequestEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for SubscriptionRequestEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SubscriptionRequestEncoder<'a> {
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

        /// primitive field 'timestampNanos'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn timestamp_nanos(&mut self, value: i64) {
            let offset = self.offset;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'id'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn id(&mut self, value: i64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'subscriberIndex'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 16
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn subscriber_index(&mut self, value: i32) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// primitive field 'securityIndex'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 20
        /// - encodedLength: 4
        /// - version: 0
        #[inline]
        pub fn security_index(&mut self, value: i32) {
            let offset = self.offset + 20;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn venue(&mut self, value: venue::Venue) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn subscription_flag(&mut self, value: subscription_flag::SubscriptionFlag) {
            let offset = self.offset + 25;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SubscriptionRequestDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> ActingVersion for SubscriptionRequestDecoder<'a> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for SubscriptionRequestDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for SubscriptionRequestDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> SubscriptionRequestDecoder<'a> {
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
        pub fn timestamp_nanos(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 8)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn subscriber_index(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 16);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn security_index(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 20);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn venue(&self) -> venue::Venue {
            self.get_buf().get_u8_at(self.offset + 24).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn subscription_flag(&self) -> subscription_flag::SubscriptionFlag {
            self.get_buf().get_u8_at(self.offset + 25).into()
        }

    }

} // end decoder


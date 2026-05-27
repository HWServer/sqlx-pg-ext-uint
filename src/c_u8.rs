use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
};

const PG_EXT_TYPE: &str = "uint1";
const PG_ARRAY_EXT_TYPE: &str = "uint1[]";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U8 {
    value: u8,
}

impl From<u8> for U8 {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<U8> for u8 {
    fn from(value: U8) -> Self {
        value.value
    }
}

impl U8 {
    pub fn get_type_size() -> usize {
        std::mem::size_of::<u8>()
    }
}

impl<'q> Encode<'q, Postgres> for U8 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn StdError + Send + Sync + 'static>> {
        let bytes = self.value.to_be_bytes();
        if bytes.len() != Self::get_type_size() {
            return Err(format!(
                "Invalid size for u8, data_len: {}, expected: {}",
                bytes.len(),
                Self::get_type_size()
            )
            .into());
        }
        <[u8; _] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }
    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for U8 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <[u8; _] as Decode<Postgres>>::decode(value)?;
        if bytes.len() != Self::get_type_size() {
            return Err(format!(
                "Invalid size for u8, data_len: {}, expected: {}",
                bytes.len(),
                Self::get_type_size()
            )
            .into());
        }
        Ok(u8::from_be_bytes(bytes).into())
    }
}

impl Type<Postgres> for U8 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_EXT_TYPE)
    }
}

impl PgHasArrayType for U8 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_ARRAY_EXT_TYPE)
    }
}

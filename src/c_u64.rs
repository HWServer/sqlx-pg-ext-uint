use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

const PG_EXT_TYPE: &str = "uint8";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U64 {
    value: u64,
}

impl From<u64> for U64 {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<U64> for u64 {
    fn from(value: U64) -> Self {
        value.value
    }
}

impl U64 {
    pub fn get_type_size() -> usize {
        std::mem::size_of::<u64>()
    }
}

impl<'q> Encode<'q, Postgres> for U64 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        let bytes = self.value.to_be_bytes();
        <[u8; _] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }
    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for U64 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <[u8; _] as Decode<Postgres>>::decode(value)?;
        Ok(u64::from_be_bytes(bytes).into())
    }
}

impl Type<Postgres> for U64 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_EXT_TYPE)
    }
}

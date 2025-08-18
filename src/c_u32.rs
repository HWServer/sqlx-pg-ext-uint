use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

const PG_EXT_TYPE: &str = "uint4";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U32 {
    value: u32,
}

impl From<u32> for U32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<U32> for u32 {
    fn from(value: U32) -> Self {
        value.value
    }
}

impl U32 {
    pub fn get_type_size() -> usize {
        std::mem::size_of::<u32>()
    }
}

impl<'q> Encode<'q, Postgres> for U32 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        let bytes = self.value.to_be_bytes();
        if bytes.len() != Self::get_type_size() {
            return Err(format!("Invalid size for u32, data_len: {}, expected: {}", bytes.len(), Self::get_type_size()).into());
        }
        <[u8; _] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }
    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for U32 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <[u8; _] as Decode<Postgres>>::decode(value)?;
        if bytes.len() != Self::get_type_size() {
            return Err(format!("Invalid size for u32, data_len: {}, expected: {}", bytes.len(), Self::get_type_size()).into());
        }
        Ok(u32::from_be_bytes(bytes).into())
    }
}

impl Type<Postgres> for U32 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_EXT_TYPE)
    }
}

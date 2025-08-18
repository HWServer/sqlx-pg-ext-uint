use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

const PG_EXT_TYPE: &str = "uint2";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U16 {
    value: u16,
}

impl From<u16> for U16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<U16> for u16 {
    fn from(value: U16) -> Self {
        value.value
    }
}

impl U16 {
    pub fn get_type_size() -> usize {
        std::mem::size_of::<u16>()
    }
}

impl<'q> Encode<'q, Postgres> for U16 {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        let bytes = self.value.to_be_bytes();
        if bytes.len() != Self::get_type_size() {
            return Err(format!("Invalid size for u16, data_len: {}, expected: {}", bytes.len(), Self::get_type_size()).into());
        }
        <[u8; _] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }
    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for U16 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes = <[u8; _] as Decode<Postgres>>::decode(value)?;
        if bytes.len() != Self::get_type_size() {
            return Err(format!("Invalid size for u16, data_len: {}, expected: {}", bytes.len(), Self::get_type_size()).into());
        }
        Ok(u16::from_be_bytes(bytes).into())
    }
}

impl Type<Postgres> for U16 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_EXT_TYPE)
    }
}

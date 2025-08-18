use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

const PG_EXT_TYPE: &str = "uint8";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct USize {
    value: usize,
}

impl From<usize> for USize {
    fn from(value: usize) -> Self {
        Self { value }
    }
}

impl From<USize> for usize {
    fn from(value: USize) -> Self {
        value.value
    }
}

impl USize {
    pub fn get_type_size() -> usize {
        std::mem::size_of::<usize>()
    }
}

impl<'q> Encode<'q, Postgres> for USize {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        // 将usize转换为大端字节序存储
        let bytes = self.value.to_be_bytes();
        <[u8; _] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }

    fn size_hint(&self) -> usize {
        Self::get_type_size()
    }
}

impl<'r> Decode<'r, Postgres> for USize {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        // 从大端字节序解析为usize
        let bytes = <[u8; _] as Decode<Postgres>>::decode(value)?;
        Ok(usize::from_be_bytes(bytes).into())
    }
}

impl Type<Postgres> for USize {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_EXT_TYPE)
    }
}

use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

/// Wrapper 一层属于自己的 crate, 不然成孤儿了
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct USize(pub usize);

impl USize {
    pub fn get_value(&self) -> usize {
        self.0
    }
}

/// 你的pg扩展类型名称，必须与数据库里类型名一致
/// 这个拓展要 uint128 的
const PG_UINT8_TYPE: &str = "uint8";
const DATA_LEN: usize = 8;

impl<'q> Encode<'q, Postgres> for USize {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<(dyn StdError + Send + Sync + 'static)>> {
        let bytes = self.0.to_be_bytes();
        <[u8; DATA_LEN] as Encode<Postgres>>::encode_by_ref(&bytes, buf)
    }
    fn size_hint(&self) -> usize {
        DATA_LEN
    }
}

impl<'r> Decode<'r, Postgres> for USize {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        // 这里解码为 [u8; 16]
        let bytes = <[u8; 8] as Decode<Postgres>>::decode(value)?;
        Ok(USize(usize::from_be_bytes(bytes)))
    }
}

impl Type<Postgres> for USize {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_UINT8_TYPE)
    }
}

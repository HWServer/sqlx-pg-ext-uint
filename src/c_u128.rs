use std::error::Error as StdError;

use sqlx::{
    Decode, Encode, Postgres, Type,
    encode::IsNull,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
};

/// Wrapper 一层属于自己的 crate, 不然成孤儿了
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U128(pub u128);

impl U128 {
    pub fn get_value(&self) -> u128 {
        self.0
    }
}

/// 你的pg扩展类型名称，必须与数据库里类型名一致
/// 这个拓展要 uint128 的
const PG_UINT16_TYPE: &str = "uint16";
const DATA_LEN: usize = 16;

impl<'q> Encode<'q, Postgres> for U128 {
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

impl<'r> Decode<'r, Postgres> for U128 {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        // 这里解码为 [u8; 16]
        let bytes = <[u8; DATA_LEN] as Decode<Postgres>>::decode(value)?;
        Ok(U128(u128::from_be_bytes(bytes)))
    }
}

impl Type<Postgres> for U128 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name(PG_UINT16_TYPE)
    }
}

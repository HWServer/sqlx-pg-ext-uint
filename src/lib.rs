pub mod c_u128;
pub mod c_u16;
pub mod c_u32;
pub mod c_u64;
pub mod c_u8;
pub mod c_usize;

/// test
#[cfg(test)]
pub mod test {
    use sqlx::{self, Row};
    use tokio;

    use crate::c_u128::U128;

    #[tokio::test]
    pub async fn main() {
        let pool = sqlx::PgPool::connect("postgres://root:xxxxxx@xxxxxx:xxxxxx/postgres")
            .await
            .unwrap();

        // use c_u128
        // extension for uint128

        sqlx::query("CREATE EXTENSION IF NOT EXISTS uint128;")
            .execute(&pool)
            .await
            .expect("Failed to create extension");
        // create table
        sqlx::query("CREATE TABLE IF NOT EXISTS test (id uint16 PRIMARY KEY, data uint16[]);")
            .execute(&pool)
            .await
            .expect("Failed to create table");
        // insert data
        // unsigned 16 bit integer
        // 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
        let id = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128;
        let data = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128 - 1;
        let data2 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128 - 2;
        sqlx::query("INSERT INTO test (id, data) VALUES ($1, $2)")
            .bind(U128::from(id))
            .bind(vec![U128::from(data), U128::from(data2)])
            .execute(&pool)
            .await
            .expect("Failed to insert data");

        // select data
        let res = sqlx::query("SELECT id, data FROM test WHERE id = $1")
            .bind(U128::from(id))
            .fetch_one(&pool)
            .await
            .expect("Failed to select data");
        println!(
            "id: {:?}, data: {:?}",
            res.get::<U128, _>(0),
            res.get::<Vec<U128>, _>(1)
        );

        // clean
        sqlx::query("DROP TABLE test;")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DROP EXTENSION uint128;")
            .execute(&pool)
            .await
            .unwrap();
    }
}

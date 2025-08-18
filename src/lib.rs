pub mod c_u128;
pub mod c_usize;

/// test
#[cfg(test)]
pub mod test {
    use sqlx::{self, Row};
    use tokio;

    use crate::c_u128::U128;

    #[tokio::test]
    pub async fn main() {
        let pool = sqlx::PgPool::connect(
            "postgres://root:TeamNovaCollaboration@192.168.2.254:65500/postgres",
        )
        .await
        .unwrap();

        // use c_u128
        // extension for uint128

        sqlx::query("CREATE EXTENSION IF NOT EXISTS uint128;")
            .execute(&pool)
            .await
            .expect("Failed to create extension");
        // create table
        sqlx::query("CREATE TABLE IF NOT EXISTS test (id uint16 PRIMARY KEY, name text);")
            .execute(&pool)
            .await
            .expect("Failed to create table");
        // insert data
        // unsigned 16 bit integer
        // 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
        sqlx::query("INSERT INTO test (id, name) VALUES ($1, $2)")
            .bind(U128::from(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128))
            .bind("test")
            .execute(&pool)
            .await
            .expect("Failed to insert data");

        // select data
        let res = sqlx::query("SELECT id, name FROM test WHERE name = $1")
            .bind("test")
            .fetch_one(&pool)
            .await
            .expect("Failed to select data");

        let id = res.get::<U128, _>("id");
        println!("id: {}", u128::from(id));
    }
}

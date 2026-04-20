use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/gsdemo".into());

    let db = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Create schema and table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sample (
            id   SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            value TEXT NOT NULL
        )
        "#,
    )
    .execute(&db)
    .await
    .expect("Failed to create table");

    // Insert sample data (idempotent via ON CONFLICT)
    let rows = [
        (1, "alpha", "Hello from alpha"),
        (2, "beta", "Hello from beta"),
        (3, "gamma", "Hello from gamma"),
    ];

    for (id, name, value) in rows {
        sqlx::query(
            r#"
            INSERT INTO sample (id, name, value)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO NOTHING
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(value)
        .execute(&db)
        .await
        .expect("Failed to insert sample data");
    }

    println!("Migrations complete: table 'sample' with {} rows", rows.len());
}

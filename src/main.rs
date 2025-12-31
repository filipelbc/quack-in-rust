use duckdb::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    //show_extensions(&conn)?;

    query_parquet_local(&conn)?;

    //query_parquet_remote(&conn)?;

    show_extensions(&conn)?;

    Ok(())
}

fn show_extensions(conn: &Connection) -> Result<()> {
    let data = conn
        .prepare("SELECT extension_name FROM duckdb_extensions() where installed")?
        .query_map([], |row| {
            let a: String = row.get(0)?;
            Ok(a)
        })?
        .collect::<Result<Vec<_>>>()?;

    println!("# extensions");

    for item in data {
        println!("{}", item);
    }

    println!("");

    Ok(())
}

fn query_parquet_local(conn: &Connection) -> Result<()> {
    let data = conn
        .prepare("SELECT * FROM '../data.parquet'")?
        .query_map([], |row| {
            let a: i32 = row.get(0)?;
            let b: String = row.get(1)?;
            Ok(format!("{}, {}", a, b))
        })?
        .collect::<Result<Vec<_>>>()?;

    println!("# parquet");

    for item in data {
        println!("{}", item);
    }

    println!("");

    Ok(())
}

fn query_parquet_remote(conn: &Connection) -> Result<()> {
    conn.execute(
        r#"
        CREATE OR REPLACE SECRET secret (
            TYPE s3,
            PROVIDER config,
            KEY_ID ?,
            SECRET ?,
            REGION ?
        );
    "#,
        [
            "id",
            "secret",
            "region",
        ],
    )?;

    let data = conn
        .prepare("SELECT * FROM 's3://filipelbc-test/data.parquet'")?
        .query_map([], |row| {
            let a: i32 = row.get(0)?;
            let b: String = row.get(1)?;
            Ok(format!("{}, {}", a, b))
        })?
        .collect::<Result<Vec<_>>>()?;

    println!("# parquet");

    for item in data {
        println!("{}", item);
    }

    println!("");

    Ok(())
}

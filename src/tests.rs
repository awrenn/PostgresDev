#[cfg(test)]
mod tests {
    use crate::PostgresContainer;

    #[test]
    fn test_connection() {
        let pc = PostgresContainer::new("postgres5434", 5434).unwrap();
        let conn = pc.heavy_connect(
            30,
            std::time::Duration::new(3, 0),
        )
        .unwrap();
        for row in &conn
            .query("SELECT * FROM pg_catalog.pg_tables", &[])
            .unwrap()
        {
            println!("Found table: {:?}", row);
        }
    }

}

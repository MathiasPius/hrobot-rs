use serde::Serialize;
use tabled::{Table, Tabled};

pub fn print_table<T>(data: &[T]) -> Result<(), Box<dyn std::error::Error>>
where
    T: Tabled,
{
    if data.is_empty() {
        println!("No results found");
        return Ok(());
    }

    let table = Table::new(data).to_string();
    println!("{}", table);
    Ok(())
}

pub fn print_table_or_json<T>(
    data: &[T],
    format: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Tabled + Serialize,
{
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(data)?;
            println!("{}", json);
            Ok(())
        }
        _ => print_table(data),
    }
}

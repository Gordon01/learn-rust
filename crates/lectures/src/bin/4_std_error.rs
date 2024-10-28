use std::{fs::File, net::Ipv4Addr};

#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("неизвестная ошибка")]
    Unknown,
    #[error("ошибка ввода-вывода: {0}")]
    IO(#[from] std::io::Error),
}

// Только напечатает отладочное представление конкретного типа,
// a `fn main() -> anyhow::Result<()>` использует `anyhow` для печати любых
// ошибок, и завершения с кодом `-1`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _: Ipv4Addr = "0.0.0".parse()?;
    process_file("точно неверное имя")?;

    Ok(())
}

fn process_file(name: &str) -> Result<(), MyError> {
    let _ = File::open(name)?;
    Err(MyError::Unknown)
}

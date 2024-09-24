use std::{fs::File, net::Ipv4Addr};

use miette::{IntoDiagnostic, WrapErr};

fn main() -> miette::Result<()> {
    process_file("точно неверное имя.json")?;

    Ok(())
}

fn process_file(name: &str) -> miette::Result<()> {
    let _ = File::open(name)
        .into_diagnostic()
        .wrap_err(format!("При открытии файла '{name}'"))?;
    let _: Ipv4Addr = "0.0.0"
        .parse()
        .into_diagnostic()
        .wrap_err("При чтении IPv4 из строки")?;

    miette::bail!("обработка не реализована")
}

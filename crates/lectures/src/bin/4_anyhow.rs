use std::{fs::File, net::Ipv4Addr};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    process_file("точно неверное имя")?;

    Ok(())
}

fn process_file(name: &str) -> anyhow::Result<()> {
    let _ = File::open(name).context(format!("открытие файла '{name}'"))?;
    let _: Ipv4Addr = "0.0.0".parse().context("чтение IPv4 из строки")?;

    anyhow::bail!("обработка не реализована")
}

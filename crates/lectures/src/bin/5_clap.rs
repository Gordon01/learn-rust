use clap::Parser;

#[derive(Parser, Debug)]
struct Person {
    /// Имя пользователя
    #[arg(short, long)]
    name: String,

    /// Возраст
    #[arg(short, long)]
    age: u8,
}

fn main() {
    let person = Person::parse();
    println!("{person:?}");
}

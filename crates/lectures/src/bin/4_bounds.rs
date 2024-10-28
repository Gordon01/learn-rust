use std::fmt::Display;

fn main() {
    let i = 42;
    print(i);
    print_w_where(i);
    print_short(i);
}

fn print(value: impl Display) {
    println!("{value}");
}

fn print_w_where<D>(value: D)
where
    D: Display,
{
    // Сама `Display` не дает методов, но зато автоматически предоставляет
    // `ToString`, в котором уже есть видимый метод `to_string()`
    println!("{}", value.to_string());
}

fn print_short<D: Display>(value: D) {
    println!("{value}");
}

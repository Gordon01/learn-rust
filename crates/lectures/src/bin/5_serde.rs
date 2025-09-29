use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Показать еще:
// #[serde(rename = "full_name")]
// #[serde(default)]
// Option

fn main() -> anyhow::Result<()> {
    let person = Person {
        name: "Ира".into(),
        age: 20,
    };

    // В JSON
    let json = serde_json::to_string(&person)?;
    println!("JSON:\n{json}");

    // В TOML
    let toml = toml::to_string(&person)?;
    println!("TOML:\n{toml}");

    // Обратно в Rust
    let deserialized_person: Person = serde_json::from_str(&json)?;
    println!("{:?}", deserialized_person);

    assert_eq!(deserialized_person, person);

    Ok(())
}

// Обычно все используют алиасы из `std`, даже на типы, которые находятся в `core`.
// Для совместимости с `no_std` (если это нужно) нужно указывать крейт точнее.
// Для clippy есть соответствующие линты, но они разрешены по умолчанию: 
// `alloc_instead_of_core`, `std_instead_of_alloc`, `std_instead_of_core`.
//
// `std` легче читается и позволяет объединять импорты. Поэтому, лучше всегда 
// использовать его. Если в будущем нужно будет переделать код под `no_std`, 
// это можно будет сделать автоматически, так как указанные выше линты 
// поддерживают автоприменение через `cargo clippy --fix` 

use std::num::NonZeroU64;

fn main() {
    let size_option = size_of::<Option<u64>>();
    let size_str = size_of::<u64>();
    println!("Option<T> size: {size_option}, T size: {size_str}");

    let size_option = size_of::<Option<&Vec<u64>>>();
    let size_str = size_of::<&Vec<u64>>();
    println!("Option<&T> size: {size_option}, &T size: {size_str}");

    let size_option = size_of::<Option<NonZeroU64>>();
    let size_str = size_of::<NonZeroU64>();
    println!("Option<NonZeroU64> size: {size_option}, NonZeroU64 size: {size_str}");
}

[clap](https://docs.rs/clap/latest/clap/) - это крейт для обработки аргументов командной строки. Один из самых известных и популярных на crates.io. Именно он используется в `cargo`, разрабатывается и поддерживается их командой, то есть одними из ключевых разработчиков Rust. sudo для Windows тоже [использует](https://github.com/microsoft/sudo/blob/9c53e97eb4fc58c9e135045946881123e0851528/Cargo.toml#L29) эту библиотеку.
Одним из основных преимуществ является использования паттерна `derive` или процедурного макроса для генерации кода из описаний типов (чаще всего - структур). Процедурный макрос - это функция, которая принимает исходный код и возвращает обработанный. Вы уже использовали `derive` для генерации кода характеристики `Default` в задании [калькулятор](%D0%9A%D0%B0%D0%BB%D1%8C%D0%BA%D1%83%D0%BB%D1%8F%D1%82%D0%BE%D1%80.md).

[Декларативное программирование](https://otus.ru/journal/imperativnoe-i-deklarativnoe-programmirovanie/) для описания аргументов командной строки отлично подходит и упрощает всю задачу до двух шагов: описания аргументов в виде структуры:

```rust
#[derive(Parser, Debug)]
struct Args {
    /// Имя
    #[arg(short, long)]
    name: String,
}
```

и вызову `let args = Args::parse();`.

Остальная работа (например, проверка типов, зависимостей между аргументами, вхождения значений в диапазоны, генерация справки, подсказки при опечатках, выделение цветом и многое другое) выполняется библиотекой. Благодаря этому структура с аргументами существует только в корректно разобранном виде (или не существует вовсе, так как `parse()` вызывает `exit()` при ошибках).

Такой подход упрощает код, уменьшает его объём и позволяет выполнять больше проверок ещё на этапе компиляции, что соответствует принципам, описанным в [этой статье](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

Если запустить программу, использующую описание выше, и выполнить команду `cargo r -q -- --help`, результат будет следующим:

```
Usage: pg.exe --name <NAME>

Options:
  -n, --name <NAME>  Имя
  -h, --help         Print help
```

`clap` использует `Option` для обработки случаев, когда значение аргумента может отсутствовать. Например, необязательный аргумент:

```rust
#[derive(Parser, Debug)]
struct Args {
    /// Имя
    #[arg(short, long)]
    name: String,

    /// Возраст
    #[arg(short, long)]
    age: Option<u8>,
}
```

В данном случае `--age` становится необязательным, и справка (`cargo r -q -- --help`) будет выглядеть так:

```
Usage: pg.exe [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>  Имя
  -a, --age <AGE>    Возраст
  -h, --help         Print help
```

Обратите внимание, что появился список `[OPTIONS]`.

Добавим обработку введённых значений:

```rust
print!("{}", args.name);
if let Some(age) = args.age {
    print!(", {age}");
}
println!(".");
```

Вот как это будет работать:

```
❯ cargo r -q                      
error: the following required arguments were not provided:
  --name <NAME>

❯ cargo r -q -- --name "Михаил Носов"
Михаил Носов.

❯ cargo r -q -- --name "Михаил Носов" --age 42
Михаил Носов, 42.

❯ cargo r -q -- --age 42                      
error: the following required arguments were not provided:
  --name <NAME>
```

## Задание

Написать утилиту `random-file` для выбора случайного файла или директории в указанной директории.

### Список принимаемых аргументов:

- **`--path <PATH>`** _(обязательный)_: Директория для поиска.
- **`--type <file|dir>`** _(опциональный)_: Искать только файлы (`file`), только директории (`dir`) или всё (`all`, значение по умолчанию).
- **`--recursive`** _(опциональный)_: Искать в подкаталогах.

Для поля `--type` понадобятся две возможности `clap`: использование [`ValueEnum`](https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_3/index.html#enumerated-values) и переименование аргумента:

```rust
struct Args {
	/// Искать только файлы, только директории или всё 
	#[arg(short, long = "type", value_enum, default_value = "all")] 
	type_filter: TypeFilter,
}

#[derive(clap::ValueEnum, Debug, Clone)] 
enum FileType { 
	File, 
	Dir, 
	All, 
}
```

### Полезные крейты

- [rand](https://docs.rs/rand/latest/rand/) для генерации случайных чисел
- [walkdir](https://docs.rs/walkdir/latest/walkdir/) для обхода директорий

### Советы

Для фильтрации по типу можно использовать сопоставление с образцом, например:

```rust
match (file_type, entry.is_file(), entry.is_dir()) { 
	(FileType::File, true, false) => {}, 
	(FileType::Dir, false, true) => {}, 
	(FileType::All, ..) => {}, 
	_ => {} 
}
```

### Примеры работы

Получившаяся программа должна демонстрировать подобное поведение:

```
❯ cargo r -q -- --path .                       
.\Cargo.toml

❯ cargo r -q -- --path target/debug 
target/debug\.fingerprint

❯ cargo r -q -- --path target/debug 
target/debug\build

❯ cargo r -q -- --path . --recursive --type dir
.\.git\info

❯ cargo r -q -- --path . --recursive --type dir
.\target\debug\.fingerprint\getrandom-cbc680de5e5b4811
```

/// Подробнее о гибкости функций и использовании обобщённых типов (generics)
/// в Rust можно прочитать в "руководстве по идиомам" https://clck.ru/3GS2d3

fn do_math_iter(it: impl Iterator<Item = u32>) -> u32 {
    it.fold(0, |acc, i| {
        let added = if i % 2 != 0 { 0 } else { i };
        acc + added
    })
}

fn do_math_better(v: &[u32]) -> u32 {
    v.iter().fold(0, |acc, i| {
        let added = if i % 2 != 0 { 0 } else { *i };
        acc + added
    })
}

fn do_math(v: &Vec<u32>) -> u32 {
    v.iter().fold(0, |acc, i| {
        let added = if i % 2 != 0 { 0 } else { *i };
        acc + added
    })
}

fn main() {
    let first = vec![0, 1, 2, 3];
    let second = vec![4, 5, 6, 7];

    println!("Сумма четных для {:?} равна {}", first, do_math(&first));
    println!("Сумма четных для {:?} равна {}", second, do_math(&second));

    let third = [10, 5, 13, 8, 12];
    // Не скомпилируется, другой тип
    // println!("Сумма четных для {:#?} равна {}", third, do_math(&third));

    println!(
        "Сумма четных для {:?} равна {}",
        first,
        do_math_better(&first)
    );
    println!(
        "Сумма четных для {:?} равна {}",
        third,
        do_math_better(&third)
    );
    println!(
        "Сумма четных для {:?} равна {}",
        second,
        do_math_better(&second)
    );

    println!("sum = {}", do_math_iter(first.into_iter()));
}

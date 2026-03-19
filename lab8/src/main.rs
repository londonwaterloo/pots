use std::cmp::Ordering;

/// Возвращает минимальное из двух значений.
///
/// Функция обобщённая (generic) и работает для любых типов,
/// которые реализуют трейт `Ord` (полный порядок).
///
/// # Аргументы
/// - a: первое значение
/// - b: второе значение
///
/// # Возвращает
/// Меньшее из двух значений
fn min<T: Ord>(a: T, b: T) -> T {
    // Сравниваем значения с помощью метода cmp,
    // который возвращает Ordering (Less, Equal, Greater)
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

fn main() {
    // Проверка на числах
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    // Проверка на символах
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    // Проверка на строках (лексикографическое сравнение)
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");

    println!("Все тесты пройдены!");
}
use std::collections::HashMap;
use std::hash::Hash;

/// Counter определяет количество значений типа T в коллекции.
///
/// Обобщённая структура, которая может работать с любыми типами,
/// если они реализуют Eq и Hash (необходимо для использования в HashMap).
struct Counter<T> {
    values: HashMap<T, u64>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    /// Создает новый пустой счетчик.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Увеличивает счетчик для заданного значения.
    fn count(&mut self, value: T) {
        // entry API — самый идиоматичный способ в Rust
        *self.values.entry(value).or_insert(0) += 1;
    }

    /// Возвращает количество появлений значения.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();

    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("Значение {} видели {} раз", i, ctr.times_seen(i));
    }

    let mut strctr = Counter::new();

    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");

    println!("Получили {} яблок", strctr.times_seen("apple"));
}
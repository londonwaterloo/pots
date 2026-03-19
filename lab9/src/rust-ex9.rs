use std::collections::HashMap;

/// Counter определяет количество знаяений типа T в коллекции.
struct Counter {
    values: HashMap<u32, u64>,
}

impl Counter {
    /// Создаем новый счетчие Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Считает количество появлений заданного значения.
    fn count(&mut self, value: u32) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Возвращает количество появлений заданного значения.
    fn times_seen(&self, value: u32) -> u64 {
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
        println!("Значение {} видели {} раз", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("Получили {} яблок", strctr.times_seen("apple"));
}
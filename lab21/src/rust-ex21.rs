use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    // left_fork: ... левая вилка
    // right_fork: ... правая вилка
    // thoughts: ... думает
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Взять вилки ...
        println!("{} ест ...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] =
    &["Сократ", "Гипатия", "Платон", "Aристотель", "Пифагор"];

fn main() {
    // Создать вилки

    // Создать философов

    // Дать им поесть и подумать 100 раз

    // Вывести их мысли
}
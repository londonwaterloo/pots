pub trait Logger {
    /// Логирует сообщение указанного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

/// Простой логгер, который выводит всё в stderr без фильтрации.
struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Логгер-фильтр, который пропускает только сообщения
/// с уровнем не выше max_verbosity.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

/// Реализация фильтрации по уровню verbosity.
impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        // Если уровень сообщения допустим —
        // передаём его во внутренний логгер
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
        // Иначе сообщение игнорируется
    }
}

fn main() {
    // Создаём фильтр с максимальным уровнем 3
    let logger = VerbosityFilter {
        max_verbosity: 3,
        inner: StderrLogger,
    };

    println!("--- Проверка VerbosityFilter ---");

    // Эти сообщения ДОЛЖНЫ пройти фильтр
    logger.log(1, "Очень важное сообщение");
    logger.log(3, "Граничный уровень");

    // Эти сообщения НЕ должны выводиться
    logger.log(4, "Слишком высокий уровень");
    logger.log(5, "Ещё одно сообщение, которое не пройдет");
}
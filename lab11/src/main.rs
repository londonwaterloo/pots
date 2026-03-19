pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

/// Простой логгер, который печатает все сообщения в stderr.
struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Фильтр-логгер.
///
/// Хранит:
/// - inner: внутренний логгер
/// - predicate: замыкание, которое решает, нужно ли пропускать сообщение
struct Filter<L, F> {
    inner: L,
    predicate: F,
}

impl<L, F> Filter<L, F> {
    /// Создает новый фильтр.
    fn new(inner: L, predicate: F) -> Self {
        Filter { inner, predicate }
    }
}

impl<L, F> Logger for Filter<L, F>
where
    L: Logger,
    F: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        // Если предикат разрешает логирование,
        // передаем сообщение внутреннему логгеру.
        if (self.predicate)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    // Пропускаем только сообщения, содержащие слово "yikes".
    let logger = Filter::new(
        StderrLogger,
        |_verbosity: u8, msg: &str| msg.contains("yikes"),
    );

    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// Описывает пакет программного обеспечения.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Возвращает этот пакет как зависимость необходимую для
    /// компиляции другого пакета.
    fn as_dependency(&self) -> Dependency {
        todo!("1")
    }
}

/// Компилятор пакета Package. Использует метод build() для создания пакета Package
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        todo!("2")
    }

    /// Задает версию пакета.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Задает автора пакета.
    fn authors(mut self, authors: Vec<String>) -> Self {
        todo!("3")
    }

    /// Добавляет зависимость.
    fn dependency(mut self, dependency: Dependency) -> Self {
        todo!("4")
    }

    /// Задает язык. Если не указан язык, используется значение по умолчанию None.
    fn language(mut self, language: Language) -> Self {
        todo!("5")
    }

    fn build(self) -> Package {
        self.0
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}
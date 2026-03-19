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
    /// Возвращает этот пакет как зависимость, необходимую
    /// для компиляции другого пакета.
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

/// Компилятор пакета Package.
struct PackageBuilder(Package);

impl PackageBuilder {
    /// Создает новый builder с обязательным полем name.
    fn new(name: impl Into<String>) -> Self {
        PackageBuilder(Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    /// Задает версию пакета.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Задает авторов пакета.
    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Добавляет зависимость.
    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Задает язык пакета.
    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    /// Возвращает готовый пакет.
    fn build(self) -> Package {
        self.0
    }
}

/// Преобразует язык в строку.
/// Здесь мы явно используем все варианты enum Language.
fn language_to_str(language: &Language) -> &'static str {
    match language {
        Language::Rust => "Rust",
        Language::Java => "Java",
        Language::Perl => "Perl",
    }
}

/// Печатает зависимости пакета в удобном виде.
/// Здесь мы явно читаем поля Dependency.
fn print_dependencies(package: &Package) {
    if package.dependencies.is_empty() {
        println!("У пакета {} нет зависимостей", package.name);
    } else {
        println!("Зависимости пакета {}:", package.name);
        for dep in &package.dependencies {
            println!("  - {} {}", dep.name, dep.version_expression);
        }
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");

    let log = PackageBuilder::new("log")
        .version("0.4")
        .language(Language::Rust)
        .build();
    println!("log: {log:?}");

    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");

    // Явно используем поля зависимостей
    print_dependencies(&serde);

    // Явно используем все варианты Language
    println!("Поддерживаемые языки:");
    println!("- {}", language_to_str(&Language::Rust));
    println!("- {}", language_to_str(&Language::Java));
    println!("- {}", language_to_str(&Language::Perl));
}
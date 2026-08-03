use std::fs;

fn replace_in_file(path: &str) {
    let content = fs::read_to_string(path).unwrap();
    let new_content = content.replace("PgPool", "SqlitePool").replace("postgres://", "sqlite://");
    fs::write(path, new_content).unwrap();
}

fn main() {
    let files = vec![
        "/home/irshad/Projects/dev/database/src/lib.rs",
        "/home/irshad/Projects/dev/portfolio/src/service.rs",
        "/home/irshad/Projects/dev/books/src/service.rs",
        "/home/irshad/Projects/dev/api/src/lib.rs",
        "/home/irshad/Projects/dev/articles/src/service.rs",
        "/home/irshad/Projects/dev/auth/src/service.rs",
        "/home/irshad/Projects/dev/analytics/src/lib.rs",
        "/home/irshad/Projects/dev/search/src/lib.rs",
        "/home/irshad/Projects/dev/app/src/main.rs",
    ];
    for file in files {
        if std::path::Path::new(file).exists() {
            replace_in_file(file);
        }
    }
}

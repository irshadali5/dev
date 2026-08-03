#[cfg(test)]
mod tests {
    use books::BookService;

    use std::path::Path;

    #[test]
    fn test_parse_book_if_exists() {
        let service = BookService::new_offline();
        let book_dir = Path::new("/home/irshad/Projects/dev/content/books/production-grade-rust");
        if book_dir.exists() {
            let book = service.load_book_from_dir(book_dir).unwrap();
            assert_eq!(book.meta.slug, "production-grade-rust");
            assert!(!book.chapters.is_empty());
        }
    }
}

#[cfg(test)]
mod tests {
    use books::BookService;
    use std::path::Path;

    #[test]
    fn test_list_books_if_exists() {
        let service = BookService::new_offline();
        let books_dir = Path::new("content/books");
        if books_dir.exists() {
            let books = service.list_books(books_dir).unwrap();
            assert!(!books.is_empty());
        }
    }
}

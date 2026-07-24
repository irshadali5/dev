use database::repositories::BookRepository;
use domain::models::Book;
use sqlx::PgPool;

pub struct BookService {
    repo: BookRepository,
}

impl BookService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: BookRepository::new(pool),
        }
    }

    pub async fn get_reading_list(&self) -> Result<Vec<Book>, sqlx::Error> {
        self.repo.get_all_books().await
    }
}

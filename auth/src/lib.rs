pub mod crypto;
pub mod service;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_hash() {
        let hash = crypto::hash_password("password123").unwrap();
        println!("HASH_OUTPUT: {}", hash);
    }
}

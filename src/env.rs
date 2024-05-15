pub static MYSQL_HOST: &str = dotenv!("MYSQL_HOST");
pub static MYSQL_PORT: &str = dotenv!("MYSQL_PORT");
pub static MYSQL_USER: &str = dotenv!("MYSQL_USER");
pub static MYSQL_PWD: &str = dotenv!("MYSQL_PWD");
pub static LOG_LEVEL: &str = dotenv!("LOG_LEVEL");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_test() {
        println!(
            "MYSQL_HOST: {}, MYSQL_PORT: {}, MYSQL_USER: {}, MYSQL_PWD: {}",
            MYSQL_HOST, MYSQL_PORT, MYSQL_USER, MYSQL_PWD
        );

        println!("LOG_LEVEL: {}", LOG_LEVEL);
    }
}

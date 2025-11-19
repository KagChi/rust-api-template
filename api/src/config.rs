#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                eprintln!("❌ Missing environment variable: DATABASE_URL");
                std::process::exit(1);
            }
        };
        
        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("❌ SERVER_PORT must be a valid number");

        AppConfig {
            database_url,
            server_port,
        }
    }
}
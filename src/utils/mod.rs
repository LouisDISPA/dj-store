pub mod cors;
#[cfg(feature = "https")]
pub mod https;
pub mod jwt;
pub mod room_id;

/// Macro to get environment variables and exit if any are missing.
///
/// Prints a list of all missing environment variables and exits with code 1.
///
/// ```
/// required_envs!(
///    db_adress => "DATABASE_URL"
///    password => "PASSWORD"
/// );
/// ```
#[macro_export]
macro_rules! required_envs {
    ($($var:ident => $key:tt)*) => {
    let mut missing = Vec::new();

    $(
        let key = $key;
        let $var = std::env::var(key).unwrap_or_else(|_| {
            missing.push(key);
            String::new()
        });
    )*

    if !missing.is_empty() {
        eprintln!("\nMissing environment variables:");
        for key in missing {
            eprintln!("  - {}", key)
        }
        std::process::exit(1);
    }
    };
}

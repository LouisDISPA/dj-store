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

/// Get the address to listen on from the first command line argument.
///
/// If no argument is given, the default address is `0.0.0.0:3000`. \
/// If the argument is invalid, the program exits with code 1.
pub fn get_addr() -> std::net::SocketAddr {
    let mut args = std::env::args();
    let executable = args.next().unwrap();

    args.next()
        .unwrap_or_else(|| "0.0.0.0:3000".to_string())
        .parse()
        .unwrap_or_else(|_| print_usage("Invalid address and port.", &executable))
}

/// Print usage and exit with code 1.
pub fn print_usage(error: &str, executable: &str) -> ! {
    eprintln!("{}", error);
    eprintln!(
        "Usage: {} <address>:<port>",
        executable.split('/').last().unwrap()
    );
    std::process::exit(1)
}

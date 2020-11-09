//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::config;

pub fn setup(server: &str, token: &str) {
    let _ = gitlab::Gitlab::new(server, token)
        .expect("Unable to connect to server, to verify server and token.");

    config::Config {
        server: server.to_string(),
        private_token: token.to_string(),
    }
    .save();
}

pub fn info() {
    let config = config::Config::open();
    let server_status =
        match gitlab::Gitlab::new(&config.server, &config.private_token) {
            Ok(_) => "Contactable".to_string(),
            Err(error) => format!("{}", &error),
        };

    println!("{:?}", &config);
    println!("Server: {}", server_status);
}

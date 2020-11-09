//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::config;

pub struct Server {
    server: gitlab::Gitlab,
}

impl Server {
    pub fn new() -> Self {
        let config = config::Config::open();
        Server {
            server: gitlab::Gitlab::new(&config.server, &config.private_token)
                .expect("Unable to connect to server"),
        }
    }
}

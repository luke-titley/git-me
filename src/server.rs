//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::config;

#[derive(
    Debug, PartialEq, serde::Serialize, serde::Deserialize, Default, Clone,
)]
pub struct Project {
    pub id: u32,
    pub path_with_namespace: std::string::String,
    pub ssh_url_to_repo: std::string::String,
}

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

    pub fn project(&self, url: &str) -> Project {
        let pageable_endpoint = gitlab::api::projects::Projects::builder()
            .build()
            .expect("Unable to list all the project in the gitlab server");

        use gitlab::api::Query as _;
        let projects: Vec<Project> =
            gitlab::api::paged(pageable_endpoint, gitlab::api::Pagination::All)
                .query(&self.server)
                .unwrap();

        for project in projects.iter() {
            if &project.ssh_url_to_repo == url {
                return project.clone();
            }
        }

        panic!("Unable to find gitlab project for current repo");
    }
}

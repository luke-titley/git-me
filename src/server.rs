//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::config;

#[derive(
    Debug, PartialEq, serde::Serialize, serde::Deserialize, Default, Clone,
)]
pub struct Project {
    pub id: u64,
    pub path_with_namespace: std::string::String,
    pub ssh_url_to_repo: std::string::String,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct User {
    pub username: std::string::String,
    pub id: u64,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct Branch {
    pub name: std::string::String,
    pub commit: Commit,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct Commit {
    pub id: std::string::String,
}

impl std::cmp::Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.username.cmp(&other.username)
    }
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
                .expect("List projects query failed");

        for project in projects.iter() {
            if &project.ssh_url_to_repo == url {
                return project.clone();
            }
        }

        panic!("Unable to find gitlab project for current repo");
    }

    pub fn find_user(&mut self, username: &str) -> User {
        let pageable_endpoint = gitlab::api::users::Users::builder()
            .build()
            .expect("Unable to list all the users in the gitlab server");

        use gitlab::api::Query as _;
        let mut users: Vec<User> =
            gitlab::api::paged(pageable_endpoint, gitlab::api::Pagination::All)
                .query(&self.server)
                .expect("List users query failed");

        for user in users.iter() {
            if &user.username == username {
                return user.clone();
            }
        }

        println!("Unable to find user '{}' users are:", username);
        users.sort();
        for user in users.iter() {
            println!("    {}", user.username);
        }

        panic!("Unable to find user")
    }

    pub fn find_head_commit(
        &mut self,
        url: &str,
        name: &str,
    ) -> std::string::String {
        let project = self.project(url);

        let endpoint =
            gitlab::api::projects::repository::branches::Branches::builder()
                .project(project.id)
                .build()
                .expect("Unable to list all the branches in the given project");

        use gitlab::api::Query as _;
        let branches: Vec<Branch> = endpoint
            .query(&self.server)
            .expect("List projects query failed");

        for branch in branches.iter() {
            if &branch.name == name {
                return branch.commit.id.clone();
            }
        }

        panic!("Given branch {} not found", name);
    }

    pub fn merge_request(
        &mut self,
        project: &Project,
        base: &str,
        branch: &str,
        reviewer: u64,
    ) {
        let title = format!("WIP: {}", branch);
        let endpoint =
            gitlab::api::projects::merge_requests::CreateMergeRequest::builder(
            )
            .project(project.id)
            .source_branch(branch)
            .target_branch(base)
            .title(&title)
            .assignee(reviewer)
            .build()
            .expect("Unable to list all the project in the gitlab server");

        use gitlab::api::Query as _;
        gitlab::api::ignore(endpoint)
            .query(&self.server)
            .expect("Create merge request failed");
    }
}

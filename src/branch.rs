//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::changelog;

#[derive(Clone, Copy)]
pub enum Type {
    Feature,
    Hotfix,
}

//------------------------------------------------------------------------------
pub fn find_name() -> std::string::String {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let head = repo.head().expect("Unable to find head");

    head.shorthand().expect("Head has no name").to_string()
}

//------------------------------------------------------------------------------
pub fn resolve(type_: Type, name: &str) -> std::string::String {
    match type_ {
        Type::Feature => format!("feature/{}", name),
        Type::Hotfix => format!("hotfix/{}", name),
    }
}

//------------------------------------------------------------------------------
pub fn base(type_: Type) -> &'static str {
    match type_ {
        Type::Feature => "develop",
        Type::Hotfix => "master",
    }
}

//------------------------------------------------------------------------------
pub fn branch(type_: Type, name: &str) {
    let branch_name = resolve(type_, name);
    let base = base(type_);
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let base_oid = repo
        .find_branch(base, git2::BranchType::Local)
        .expect(&format!("Unable to find branch {}", base))
        .get()
        .target()
        .expect("Unable to find reference target");
    let commit = repo
        .find_commit(base_oid)
        .expect("Unable to find head commit");
    let mut branch = repo
        .branch(&branch_name, &commit, false)
        .expect("Unable to create branch");

    branch
        .set_upstream(Some(&format!("{}", branch_name)))
        .expect("Unable to set the upstream branch");

    repo.set_head(
        branch
            .get()
            .name()
            .expect("Unable to get the refname for new branch"),
    )
    .expect("Unable to set HEAD to point to new branch");

    let changelog = changelog::create_stub(&branch_name);
    let mut index = repo.index().expect("Unable to create index for changelog");
    index
        .add_path(changelog.as_path())
        .expect("Unable to add changelog to index");

    let index_oid = index.write_tree().expect("Unable to write index");

    let tree = repo
        .find_tree(index_oid)
        .expect("Unable to find tree for new index");

    repo.commit(
        Some("HEAD"),
        &repo.signature().expect("Unable to obtain signature"),
        &repo.signature().expect("Unable to obtain signature"),
        &format!("Fork branch '{}' from '{}'", branch_name, base),
        &tree,
        &[&commit],
    )
    .expect("Unable to make initial commit");

    repo.checkout_head(None).expect("Reset everything to head");
}

/*
//------------------------------------------------------------------------------
pub fn update_all() {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");

    let mut index = repo.index().expect("Unable to create index for changelog");
    index.update_all(&["*"], None).expect("Unable to update all");
}
*/

//------------------------------------------------------------------------------
pub fn remove_from_index(paths: &[std::path::PathBuf]) {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");

    let mut index = repo.index().expect("Unable to create index for changelog");
    for path in paths.iter() {
        index
            .remove_path(path.as_path())
            .expect(&format!("Unable to add {:?} to index", &path));
    }
}

//------------------------------------------------------------------------------
pub fn add_and_remove(
    branch: &str,
    comment: &str,
    add: &[std::path::PathBuf],
    remove: &[std::path::PathBuf],
) {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");

    let mut index = repo.index().expect("Unable to create index for changelog");

    // Add
    for path in add.iter() {
        index
            .add_path(path.as_path())
            .expect(&format!("Unable to add {:?} to index", &path));
    }

    // Remove
    for path in remove.iter() {
        index
            .remove_path(path.as_path())
            .expect(&format!("Unable to add {:?} to index", &path));
    }

    let index_oid = index.write_tree().expect("Unable to write index");

    let tree = repo
        .find_tree(index_oid)
        .expect("Unable to find tree for new index");

    let base_oid = repo
        .find_branch(branch, git2::BranchType::Local)
        .expect(&format!("Unable to find branch {}", branch))
        .get()
        .target()
        .expect("Unable to find reference target");
    let commit = repo
        .find_commit(base_oid)
        .expect("Unable to find head commit");

    repo.commit(
        Some("HEAD"),
        &repo.signature().expect("Unable to obtain signature"),
        &repo.signature().expect("Unable to obtain signature"),
        comment,
        &tree,
        &[&commit],
    )
    .expect("Unable to make initial commit");

    repo.checkout_head(None).expect("Reset everything to head");
}

//------------------------------------------------------------------------------
pub fn push(branch_name: &str) {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let mut remote = repo
        .find_remote("origin")
        .expect("Unable to find remote repo");

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        git2::Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!(
                "{}/.ssh/id_rsa",
                std::env::var("HOME").unwrap()
            )),
            None,
        )
    });

    let ref_spec = format!("refs/heads/{0}:refs/heads/{0}", branch_name);
    remote
        .push(
            &[&ref_spec],
            Some(git2::PushOptions::new().remote_callbacks(callbacks)),
        )
        .expect("Failed to push");
}

pub fn find_remote() -> std::string::String {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let remote = repo
        .find_remote("origin")
        .expect("Unable to find remote repo");

    remote
        .url()
        .expect("remote url is not valid utf8")
        .to_string()
}

pub fn verify_index_empty() -> bool {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");

    let statuses = repo.statuses(None).expect("Error getting status");
    statuses.is_empty()
}

pub fn verify_up_to_date(base_commit: &str, name: &str) -> bool {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");

    let base_commit_oid = git2::Oid::from_str(base_commit)
        .expect("Unable to convert the base commit id to an Oid");

    let branch_commit = {
        let b_oid = repo
            .find_branch(name, git2::BranchType::Local)
            .expect(&format!("Unable to find branch {}", name))
            .get()
            .target()
            .expect("Unable to find reference target");

        repo.find_commit(b_oid)
            .expect(&format!("Unable to find head commit for {}", name))
    };

    let mut current_commit = branch_commit.clone();
    loop {
        if current_commit.id() == base_commit_oid {
            return true;
        }

        if current_commit.parent_count() == 0 {
            break;
        } else {
            current_commit = current_commit
                .parent(0)
                .expect("Unable to obtain parent commit");
        }
    }

    false
}

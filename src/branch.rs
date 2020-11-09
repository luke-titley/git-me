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
    let develop_oid = repo
        .find_branch(base, git2::BranchType::Local)
        .expect(&format!("Unable to find branch {}", base))
        .get()
        .target()
        .expect("Unable to find reference target");
    let commit = repo
        .find_commit(develop_oid)
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

    let changelog = changelog::create_stub(name);
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
        &format!("[start] {}", branch_name),
        &tree,
        &[&commit],
    )
    .expect("Unable to make initial commit");

    repo.checkout_head(None).expect("Reset everything to head");
}

//------------------------------------------------------------------------------
pub fn push(type_: Type, name: &str) {
    let branch_name = resolve(type_, name);
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

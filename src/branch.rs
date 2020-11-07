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
    let branch = repo
        .branch(&branch_name, &commit, false)
        .expect("Unable to create branch");

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

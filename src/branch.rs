//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
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
    let name = resolve(type_, name);
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
        .branch(&name, &commit, false)
        .expect("Unable to create branch");

    repo.set_head(
        branch
            .get()
            .name()
            .expect("Unable to get the refname for new branch"),
    )
    .expect("Unable to set HEAD to point to new branch");
}

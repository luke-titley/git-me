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
        Type::Hotfix => "develop", // Need to switch this to master
    }
}

//------------------------------------------------------------------------------
pub fn branch(type_: Type, name: &str) {
    let branch_name = resolve(type_, name);
    let base = base(type_);
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let base_branch = repo
        .find_branch(base, git2::BranchType::Local)
        .expect(&format!("Unable to find branch {}", base));
    let base_reference = base_branch.get();
    let base_oid = base_reference
        .target()
        .expect("Unable to find reference target");
    let commit = repo
        .find_commit(base_oid)
        .expect("Unable to find head commit");

    repo.set_head(
        base_reference
            .name()
            .expect("Unable to get base branch reference name"),
    )
    .expect(&format!(
        "Unable to set HEAD to point to {} before branching",
        base
    ));

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

fn find_commit(reference: &git2::Commit, commit: &git2::Commit) -> bool {
    if commit.id() == reference.id() {
        return true;
    }
    for p in commit.parents() {
        if find_commit(reference, &p) {
            return true;
        }
    }
    return false;
}

//------------------------------------------------------------------------------
pub fn rebase_place_holder(type_: Type, fullname: &str) {
    /*
    git checkout {0} && git pull --rebase && git checkout {1} && git rebase {0} && git push -f origin {1}
    */
    let base = base(type_);

    // git checkout base
    std::process::Command::new("git")
        .arg("checkout")
        .arg(base)
        .spawn()
        .expect("failed to execute git checkout")
        .wait()
        .expect("failed to wait on git checkout");

    // git pull --rebase
    std::process::Command::new("git")
        .arg("pull")
        .arg("--rebase")
        .spawn()
        .expect("failed to execute git pull")
        .wait()
        .expect("failed to wait on git pull");

    // git checkout branch
    std::process::Command::new("git")
        .arg("checkout")
        .arg(fullname)
        .spawn()
        .expect(&format!("failed to execute git checkout to {}", fullname))
        .wait()
        .expect(&format!("failed to wait on git checkout to {}", fullname));

    // git rebase develop
    std::process::Command::new("git")
        .arg("rebase")
        .arg(base)
        .spawn()
        .expect(&format!("failed to execute git rebase {} on develop", fullname))
        .wait()
        .expect(&format!("failed to wait on git checkout {} on develop", fullname));

    // git push -f origin {0}
    std::process::Command::new("git")
        .arg("push")
        .arg("-f")
        .arg("origin")
        .arg(fullname)
        .spawn()
        .expect(&format!("failed to execute git push for {}", fullname))
        .wait()
        .expect(&format!("failed to wait on git push for {}", fullname));
}

//------------------------------------------------------------------------------
pub fn rebase_not_finished(type_: Type, _fullname: &str) {
    let base = base(type_);
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let mut base_branch = repo
        .find_branch(base, git2::BranchType::Local)
        .expect(&format!("Unable to find branch {}", base));

    let base_reference = base_branch.get_mut();

    println!("    * Switch to {}", base);
    let base_reference_name = base_reference
        .name()
        .expect("Unable to get base branch reference name");

    println!("    * Fetch");
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

    remote
        .fetch(
            &[&base],
            Some(git2::FetchOptions::new().remote_callbacks(callbacks)),
            None,
        )
        .expect("Failed to fetch");

    let remote_reference = repo
        .find_reference(
            repo.branch_upstream_name(base_reference_name)
                .expect("Unable to find upstream branch")
                .as_str()
                .expect("refname is not utf8"),
        )
        .expect("Unable to obtain remote reference");
    let remote_commit = repo
        .find_commit(
            remote_reference
                .target()
                .expect("Unable to find reference target"),
        )
        .expect("Unable to find head commit");

    let base_commit = repo
        .find_commit(
            base_reference
                .target()
                .expect("Unable to find reference target"),
        )
        .expect("Unable to find head commit");

    print!("    * Verify history");

    // Ensure we can find the head commit of our local branch in the remote one
    if find_commit(&base_commit, &remote_commit) {
        println!(" ok");
    } else {
        println!(" failed");
        panic!("You have local changes on {}. You've gone rogue. Get those changes pushed through a merge request.", base);
    }

    let statuses = repo.statuses(None).expect("Error getting status");
    if !statuses.is_empty() {
        panic!("You have un commited changes on {}.", base);
    }

    println!("    * Switching to latest {}", base);
    base_reference
        .set_target(remote_commit.id(), "bump base to latest")
        .expect("error jumping base up to state of remote");

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

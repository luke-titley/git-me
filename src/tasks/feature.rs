//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
pub fn start(name: &str) {
    let repo =
        git2::Repository::discover("./").expect("Unable to find git repo");
    let head_oid = repo
        .refname_to_id("develop")
        .expect("Unable to find refname");
    let commit = repo
        .find_commit(head_oid)
        .expect("Unable to find head commit");
    repo.branch(&format!("feature/{}", name), &commit, false)
        .expect("Unable to create branch");
}

//------------------------------------------------------------------------------
pub fn review(reviewer: &str) {
    println!("review feature by {}", reviewer);
}

//------------------------------------------------------------------------------
pub fn finish() {
    println!("finish feature");
}

//------------------------------------------------------------------------------
pub fn rebase() {
    println!("rebase feature");
}

//------------------------------------------------------------------------------
pub fn enter() {
    println!("enter feature");
}

//------------------------------------------------------------------------------
pub fn exit() {
    println!("exit feature");
}

//------------------------------------------------------------------------------
pub fn switch() {
    println!("switch feature");
}

//------------------------------------------------------------------------------
pub fn list() {
    println!("list feature");
}

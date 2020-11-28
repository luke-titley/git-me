//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;
use crate::server;

//------------------------------------------------------------------------------
pub fn start(branch_type: branch::Type, name: &str, reviewer: &str) {
    // Make sure we are on the develop branch
    if branch::find_name() != branch::base(branch_type) {
        panic!(
            "You must start this new branch from {}",
            branch::base(branch_type)
        );
    }

    // Find the user specified in reviewer
    let mut server = server::Server::new();
    let assignee = server.find_user(reviewer);

    // Make the new branch
    println!("    * {}", &branch::resolve(branch_type, name));
    branch::branch(branch_type, name);

    // Push the new branch
    println!("    * push");
    branch::push(&branch::resolve(branch_type, name));

    // Create a new merge request upfront
    println!("    * wip merge request");
    let remote_url = branch::find_remote();
    let project = server.project(&remote_url);
    server.merge_request(
        &project,
        branch::base(branch_type),
        &branch::resolve(branch_type, name),
        assignee.id,
    );
}

//------------------------------------------------------------------------------
pub fn review(branch_type: branch::Type) {
    let branch_name = branch::find_name();

    // Verify the changelog has been filled out
    if !changelog::verify(&branch_name) {
        panic!("You've not filled in your changelog");
    }

    // Verify there's nothing in the index
    if !branch::verify_index_empty() {
        panic!("You have uncommited changes");
    }

    // Verify that our branch is up to speed
    let mut server = server::Server::new();
    let remote_url = branch::find_remote();
    let head_commit =
        server.find_head_commit(&remote_url, branch::base(branch_type));

    // Verify that your branch is rebased on top of the latest work in base
    if !branch::verify_up_to_date(&head_commit, &branch_name) {
        panic!(
            "Your branch is not rebased on the latest {}. You need to pull and rebase",
            branch::base(branch_type)
        );
    }

    // Push your work
    branch::push(&branch_name);
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

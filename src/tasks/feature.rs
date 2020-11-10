//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;
use crate::server;

//------------------------------------------------------------------------------
pub fn start(name: &str, reviewer: &str) {
    // Find the user specified in reviewer
    let mut server = server::Server::new();
    let assignee = server.find_user(reviewer);

    // Make the new branch
    println!("    * {}", &branch::resolve(branch::Type::Feature, name));
    branch::branch(branch::Type::Feature, name);

    // Push the new branch
    println!("    * push");
    branch::push(branch::Type::Feature, name);

    // Create a new merge request upfront
    println!("    * wip merge request");
    let remote_url = branch::find_remote();
    let project = server.project(&remote_url);
    server.merge_request(
        &project,
        branch::base(branch::Type::Feature),
        &branch::resolve(branch::Type::Feature, name),
        assignee.id,
    );
}

//------------------------------------------------------------------------------
pub fn review() {
    let branch_name = branch::find_name();

    // Verify the changelog has been filled out
    if !changelog::verify(&branch_name.replace("/", "_")) {
        panic!("You've not filled in your changelog");
    }

    // Verify there's nothing in the index
    if !branch::verify_index_empty() {
        panic!("You have uncommited changes");
    }

    // Set the reviewer
    branch::pull_base(branch::Type::Feature);
    if !branch::verify_rebased(branch::Type::Feature, &branch_name) {
        panic!("You've not rebased your branch");
    }
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

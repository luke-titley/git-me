//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;
use crate::server;

//------------------------------------------------------------------------------
pub fn start(branch_type: branch::Type, name: &str, reviewer: &str) {
    // Find the user specified in reviewer
    print!("    * does {} exist? ", reviewer);
    let mut server = server::Server::new();
    let assignee = server.find_user(reviewer);
    println!("yes");

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
    println!("    * Veryify changelog");
    if !changelog::verify(&branch_name) {
        panic!("You've not filled in your changelog");
    }

    // Verify there's nothing in the index
    println!("    * Veryify all changed committed");
    if !branch::verify_index_empty() {
        panic!("You have uncommited changes");
    }

    // Verify that our branch is up to speed
    println!("    * Veryify we're up to speed");
    let mut server = server::Server::new();
    let remote_url = branch::find_remote();
    let head_commit =
        server.find_head_commit(&remote_url, branch::base(branch_type));

    // Verify that your branch is rebased on top of the latest work in base
    if !branch::verify_up_to_date(&head_commit, &branch_name) {
        println!("        * Rebasing");
        branch::rebase_place_holder(branch_type, &branch_name);
    }

    // Push your work
    branch::push(&branch_name);
}

//------------------------------------------------------------------------------
pub fn rebase(branch_type: branch::Type) {
    branch::rebase_place_holder(branch_type, &branch::find_name());
}

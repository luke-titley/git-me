//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::server;

//------------------------------------------------------------------------------
pub fn start(name: &str) {
    branch::branch(branch::Type::Feature, name);
}

//------------------------------------------------------------------------------
pub fn review(reviewer: &str) {
    println!("review feature by {}", reviewer);
    let remote_url = branch::find_remote();
    let server = server::Server::new();

    let project = server.project(&remote_url);
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

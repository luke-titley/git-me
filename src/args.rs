//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use argh::FromArgs;

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The task that is going to be done.
pub struct Tasks {
    #[argh(subcommand)]
    pub task: Task,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Task {
    Feature(Feature),
    Hotfix(Hotfix),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(option)]
    /// start a new feature
    pub name: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "review")]
pub struct Review {
    #[argh(option)]
    /// start a new feature
    pub reviewer: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "finish")]
pub struct Finish {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Stage {
    Start(Start),
    Review(Review),
    Finish(Finish),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "feature")]
pub struct Feature {
    #[argh(subcommand)]
    /// start a new feature
    pub stage: Stage,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "hotfix")]
pub struct Hotfix {
    #[argh(subcommand)]
    /// start a new hotfix
    pub stage: Stage,
}

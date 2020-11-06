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
/// start
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(option)]
    /// name of the new feature/hotfix
    pub name: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// review
#[argh(subcommand, name = "review")]
pub struct Review {
    #[argh(option)]
    /// start a new feature
    pub reviewer: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// finish
#[argh(subcommand, name = "finish")]
pub struct Finish {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// enter
#[argh(subcommand, name = "enter")]
pub struct Enter {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// exit
#[argh(subcommand, name = "exit")]
pub struct Exit {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// switch
#[argh(subcommand, name = "switch")]
pub struct Switch {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// list
#[argh(subcommand, name = "list")]
pub struct List {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Status {
    Start(Start),
    Review(Review),
    Finish(Finish),
    Enter(Enter),
    Exit(Exit),
    Switch(Switch),
    List(List)
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Working with feature
#[argh(subcommand, name = "feature")]
pub struct Feature {
    #[argh(subcommand)]
    /// the stage in the feature
    pub status: Status,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Working with hotfix
#[argh(subcommand, name = "hotfix")]
pub struct Hotfix {
    #[argh(subcommand)]
    /// the stage in the feature
    pub status: Status,
}

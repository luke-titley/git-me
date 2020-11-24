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
    Release(Release),
    Setup(Setup),
    Info(Info),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// start
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(option)]
    /// name of the new feature/hotfix
    pub name: std::string::String,

    #[argh(option)]
    /// name of the person to review your work
    pub reviewer: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// review
#[argh(subcommand, name = "review")]
pub struct Review {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// rebase
#[argh(subcommand, name = "rebase")]
pub struct Rebase {}

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
    Rebase(Rebase),
    Enter(Enter),
    Exit(Exit),
    Switch(Switch),
    List(List),
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

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// develop
#[argh(subcommand, name = "develop")]
pub struct Develop {
    #[argh(option)]
    /// the tag version we will use for this release
    pub tag : std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// develop
#[argh(subcommand, name = "master")]
pub struct Master {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Branch {
    Master(Master),
    Develop(Develop),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Working with master branch
#[argh(subcommand, name = "release")]
pub struct Release {
    #[argh(subcommand)]
    /// the operation to perform
    pub branch: Branch,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The initial setup of git-me
#[argh(subcommand, name = "setup")]
pub struct Setup {
    #[argh(option)]
    /// the gitlab server
    pub server: std::string::String,
    #[argh(option)]
    /// api token
    pub private_token: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The initial setup of git-me
#[argh(subcommand, name = "info")]
pub struct Info {}

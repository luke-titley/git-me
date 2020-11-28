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
    Changelog(Changelog),
    Setup(Setup),
    Info(Info),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// start
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(option, short = 'n')]
    /// name of the new feature/hotfix
    pub name: std::string::String,

    #[argh(option, short = 'r')]
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
#[argh(subcommand)]
pub enum Status {
    Start(Start),
    Review(Review),
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
/// Build a changelog for a version, by merging feature changelogs
#[argh(subcommand, name = "aggregate")]
pub struct Aggregate {
    #[argh(option)]
    /// the tag version we will use for this changelog
    pub tag: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Build a changelog for a version, by merging feature changelogs
#[argh(subcommand, name = "validate")]
pub struct Validate {
    #[argh(option)]
    /// the tag version we will use for this changelog
    pub path: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum ChangelogStatus {
    Aggregate(Aggregate),
    Validate(Validate),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Operations for working with changelogs
#[argh(subcommand, name = "changelog")]
pub struct Changelog {
    #[argh(subcommand)]
    /// combine the feature changlogs
    pub status: ChangelogStatus,
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

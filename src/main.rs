use argh::FromArgs;

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The task that is going to be done.
struct Tasks {
    #[argh(subcommand)]
    task: Task,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Task {
    Feature(Feature),
    Hotfix(Hotfix),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "start")]
struct Start {
    #[argh(option)]
    /// start a new feature
    name: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "review")]
struct Review {
    #[argh(option)]
    /// start a new feature
    reviewer: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "finish")]
struct Finish {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Stage {
    Start(Start),
    Review(Review),
    Finish(Finish),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "feature")]
struct Feature {
    #[argh(subcommand)]
    /// start a new feature
    stage: Stage,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "hotfix")]
struct Hotfix {
    #[argh(subcommand)]
    /// start a new hotfix
    stage: Stage,
}

//------------------------------------------------------------------------------
fn main() {
    //let opt: Options = argh::from_env();
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        Task::Feature(Feature {
            stage: Stage::Start(Start { name }),
        }) => {
            println!("start feature/{}", name);
        },
        Task::Hotfix(Hotfix {
            stage: Stage::Start(Start { name }),
        }) => {
            println!("start feature/{}", name);
        }
        _ => (),
    }
}

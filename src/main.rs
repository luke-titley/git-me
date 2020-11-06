//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;
mod tasks;

use args::*;

//------------------------------------------------------------------------------
fn main() {
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        // Feature
        Task::Feature(Feature { status }) => match status {
            Status::Start(Start { name }) => tasks::feature::start(&name),
            Status::Review(Review { reviewer }) => tasks::feature::review(&reviewer),
            Status::Finish(Finish {}) => tasks::feature::finish(),
            Status::Rebase(Rebase {}) => tasks::feature::rebase(),
            Status::Enter(Enter {}) => tasks::feature::enter(),
            Status::Exit(Exit {}) => tasks::feature::exit(),
            Status::Switch(Switch {}) => tasks::feature::switch(),
            Status::List(List {}) => tasks::feature::list(),
        },
        Task::Hotfix(Hotfix {
            status: Status::Start(Start { name }),
        }) => {
            println!("start hotfix/{}", name);
        }
        Task::Release(Release {
            branch: Branch::Master(Master {}),
        }) => {
            println!("Release patch");
        }
        Task::Release(Release {
            branch: Branch::Develop(Develop {}),
        }) => {
            println!("Release minor");
        }
        Task::Setup(Setup { server, token }) => {
            println!("setup {} {}", server, token);
        }
        _ => (),
    }
}

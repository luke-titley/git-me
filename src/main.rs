//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;
mod branch;
mod changelog;
mod config;
mod server;
mod tasks;

use args::*;

//------------------------------------------------------------------------------
fn main() {
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        // Feature
        Task::Feature(Feature { status }) => match status {
            Status::Start(Start { name, reviewer }) => {
                tasks::feature::start(&name, &reviewer)
            }
            Status::Review(Review {}) => tasks::feature::review(),
            Status::Finish(Finish {}) => tasks::feature::finish(),
            Status::Rebase(Rebase {}) => tasks::feature::rebase(),
            Status::Enter(Enter {}) => tasks::feature::enter(),
            Status::Exit(Exit {}) => tasks::feature::exit(),
            Status::Switch(Switch {}) => tasks::feature::switch(),
            Status::List(List {}) => tasks::feature::list(),
        },
        Task::Hotfix(Hotfix {
            status: Status::Start(Start { .. }),
        }) => {
            println!("Not implemented");
        }
        Task::Changelog(Changelog {
            status: ChangelogStatus::Aggregate(Aggregate { tag }),
        }) => {
            tasks::changelog::aggregate(&tag);
        }
        Task::Changelog(Changelog {
            status: ChangelogStatus::Validate(Validate { path }),
        }) => {
            tasks::changelog::validate(&path);
        }
        Task::Setup(Setup {
            server,
            private_token,
        }) => {
            tasks::setup::setup(&server, &private_token);
        }
        Task::Info(Info {}) => {
            tasks::setup::info();
        }
        _ => (),
    }
}

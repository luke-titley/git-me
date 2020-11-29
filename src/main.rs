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
                tasks::work::start(branch::Type::Feature, &name, &reviewer)
            }
            Status::Review(Review {}) => {
                tasks::work::review(branch::Type::Feature)
            }
            Status::Rebase(Rebase {}) => {
                println!("wip");
                //tasks::work::rebase(branch::Type::Feature)
            }
        },
        // Hotfix
        Task::Hotfix(Hotfix { status }) => match status {
            Status::Start(Start { name, reviewer }) => {
                tasks::work::start(branch::Type::Hotfix, &name, &reviewer)
            }
            Status::Review(Review {}) => {
                tasks::work::review(branch::Type::Hotfix)
            }
            Status::Rebase(Rebase {}) => {
                println!("wip");
                //tasks::work::rebase(branch::Type::Hotfix)
            }
        },
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

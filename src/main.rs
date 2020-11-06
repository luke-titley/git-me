//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;
mod tasks;

use args::*;

//------------------------------------------------------------------------------
fn main() {
    //let opt: Options = argh::from_env();
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        // Feature
        Task::Feature(Feature {
            status: Status::Start(Start { name }),
        }) => {
            println!("start feature/{}", name);
        }
        Task::Hotfix(Hotfix {
            status: Status::Start(Start { name }),
        }) => {
            println!("start hotfix/{}", name);
        }
        Task::Release(Release{ branch : Branch::Master(Master{}) }) => {
            println!("Release patch");
        }
        Task::Release(Release{ branch : Branch::Develop(Develop{}) }) => {
            println!("Release minor");
        }
        Task::Setup(Setup { server, token }) => {
            println!("setup");
        }
        _ => (),
    }
}

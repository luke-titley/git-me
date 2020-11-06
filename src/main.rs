//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;

use args::{Tasks, Task, Feature, Hotfix, Start, Status};

//------------------------------------------------------------------------------
fn main() {
    //let opt: Options = argh::from_env();
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        Task::Feature(Feature {
            status: Status::Start(Start { name }),
        }) => {
            println!("start feature/{}", name);
        },
        Task::Hotfix(Hotfix {
            status: Status::Start(Start { name }),
        }) => {
            println!("start feature/{}", name);
        }
        _ => (),
    }
}

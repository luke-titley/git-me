//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;
mod tasks;

use args::{Feature, Hotfix, Start, Status, Task, Tasks};

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
            println!("start feature/{}", name);
        }
        _ => (),
    }
}

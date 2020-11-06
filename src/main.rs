//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;

use args::{Tasks, Task, Feature, Hotfix, Start, Stage};

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

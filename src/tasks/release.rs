//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;

//------------------------------------------------------------------------------
pub fn develop(tag: &str) {
    // Build the aggregate changelog
    let (aggregate_changelog, changelogs) =
        changelog::aggregate(tag, &["feature", "hotfix"]);

    // Add the new changes to the index
    branch::add_and_remove(
        "develop",
        "[changelog][regenerate]",
        &[aggregate_changelog],
        &changelogs[..],
    );
}

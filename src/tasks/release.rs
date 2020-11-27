//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::changelog;

//------------------------------------------------------------------------------
pub fn develop(tag: &str) {
    // Build the aggregate changelog
    let _ = changelog::aggregate(tag, &["feature", "hotfix"]);
}

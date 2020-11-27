//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::changelog;

//------------------------------------------------------------------------------
pub fn aggregate(tag: &str) {
    // Build the aggregate changelog
    let _ = changelog::aggregate(tag, &["feature", "hotfix"]);
}

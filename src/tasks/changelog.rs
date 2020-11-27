//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::changelog;

//------------------------------------------------------------------------------
pub fn aggregate(tag: &str) {
    // Build the aggregate changelog
    let _ = changelog::aggregate(tag, &["feature", "hotfix"]);
}

//------------------------------------------------------------------------------
pub fn validate(path: &str) {
    // Build the aggregate changelog
    if !changelog::validate(&std::path::PathBuf::from(path)) {
        panic!("Failed to validate {}", path);
    }
}

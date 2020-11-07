//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------

const ARTIST_DESCR: &'static str = "For artists";
const TECHNICAL_DESCR: &'static str = "For developers";

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Changelog<'a> {
    pub artists: &'a str,
    pub technical: &'a str,
}

impl<'a> Changelog<'a> {
    pub fn new() -> Self {
        Self {
            artists: ARTIST_DESCR,
            technical: TECHNICAL_DESCR,
        }
    }
}

//------------------------------------------------------------------------------
pub fn resolve(name: &str) -> std::path::PathBuf {
    let mut changelog_file = std::path::Path::new("changelog").to_path_buf();
    changelog_file.push(name);
    changelog_file.set_file_name(name);
    changelog_file.set_extension("yml");
    changelog_file
}

//------------------------------------------------------------------------------
pub fn create_stub(name: &str) -> std::path::PathBuf {
    // Create the changelog folder
    let changelog_dir = std::path::Path::new("changelog");
    if !changelog_dir.exists() {
        std::fs::create_dir(changelog_dir)
            .expect("Unable to create changlog folder");
    }

    // Create the changelog file path
    let mut changelog_file = changelog_dir.to_path_buf();
    changelog_file.push(name);
    changelog_file.set_file_name(name);
    changelog_file.set_extension("yml");

    // Write a stub changelog file to disk
    serde_yaml::to_writer(
        std::fs::File::create(&changelog_file)
            .expect("Unable to create changelog file"),
        &Changelog::new(),
    )
    .expect("Unable to write the changlog to disk");

    changelog_file
}

//------------------------------------------------------------------------------
pub fn verify(name: &str) -> bool {
    false
}

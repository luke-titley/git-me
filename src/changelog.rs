//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------

const ARTIST_DESCR: &'static str = "For artists";
const TECHNICAL_DESCR: &'static str = "For developers";
const CHANGELOG: &'static str = "changelog";

use serde_yaml::Value;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Changelog {
    pub artists: Value,
    pub technical: Value,
}

impl Changelog {
    pub fn new() -> Self {
        Self {
            artists: Value::String(ARTIST_DESCR.to_string()),
            technical: Value::String(TECHNICAL_DESCR.to_string()),
        }
    }
    pub fn empty() -> Self {
        Self {
            artists: Value::Null,
            technical: Value::Null,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.artists.is_null() && self.technical.is_null()
    }
}

//------------------------------------------------------------------------------
pub fn resolve(name: &str) -> std::path::PathBuf {
    let mut changelog_file = std::path::PathBuf::from(CHANGELOG);
    changelog_file.push(std::path::Path::new(name));
    changelog_file.set_extension("yml");
    changelog_file
}

//------------------------------------------------------------------------------
pub fn create_stub(name: &str) -> std::path::PathBuf {
    // Build the changelog file path
    let changelog_file = resolve(name);

    // Make sure the owning folder exists
    std::fs::create_dir_all(changelog_file.parent().unwrap())
        .expect("Unable to create changlog folder");

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
    let path = resolve(name);
    let change_log: Changelog = serde_yaml::from_reader(
        std::fs::File::open(
            path.to_str().expect("changelog path is not unicode"),
        )
        .expect("Unable to read change log file"),
    )
    .expect("Unable to parse the change log from disk");

    let are_equal = change_log == Changelog::new();

    !are_equal
}

//------------------------------------------------------------------------------
fn merge_work(lhs: &mut Value, rhs: &Value) {}

//------------------------------------------------------------------------------
pub fn aggregate(
    tag: &str,
    prefix: &[&str],
) -> (std::path::PathBuf, std::vec::Vec<std::path::PathBuf>) {
    // Obtain a list of all the changelog files that match the given prefixes.
    // These will be aggregated and combined into a single changelog.
    let mut change_logs: std::vec::Vec<std::path::PathBuf> =
        glob::glob(&format!("{}/**/*", &CHANGELOG))
            .expect("Failed to read glob")
            .filter(|e| {
                if let Ok(entry) = e {
                    if entry.is_file() {
                        let file_path = entry.to_str().unwrap();
                        for p in prefix.iter() {
                            if file_path[CHANGELOG.len() + 1..].starts_with(p) {
                                return true;
                            }
                        }
                    }
                }
                false
            })
            .map(|p| p.unwrap())
            .collect();

    change_logs.sort();
    let change_logs = change_logs;

    // Aggregate all the changelogs to produce a single one with the combined
    let mut aggregate_changelog = Changelog::empty();
    for changelog_file in change_logs.iter() {
        let changelog: Changelog = serde_yaml::from_reader(
            std::fs::File::open(&changelog_file)
                .expect("Unable to open changelog file"),
        )
        .expect("Unable to read changelog file");

        if changelog != Changelog::new() {
            // Combine all the artists notes
            if !changelog.artists.is_null() {
                merge_work(
                    &mut aggregate_changelog.artists,
                    &changelog.artists,
                );
            }

            // Combine all the technical notes
            if !changelog.technical.is_null() {
                merge_work(
                    &mut aggregate_changelog.technical,
                    &changelog.technical,
                );
            }
        }
    }

    if aggregate_changelog.is_empty() {
        panic!("There are no changelogs for this release!");
    }

    // Write the aggregate changelog to disk
    let aggregate_changelog_path = resolve(&format!("{}.e", tag));
    serde_yaml::to_writer(
        std::fs::File::create(&aggregate_changelog_path)
            .expect("Unable to create aggregate changelog file"),
        &aggregate_changelog,
    )
    .expect("Unable to write the aggregate changlog to disk");

    // Remove the individual changelogs
    for change_log in change_logs.iter() {
        std::fs::remove_file(&change_log)
            .expect(&format!("Unable to remove {:?}", &change_log));
    }

    (aggregate_changelog_path, change_logs)
}

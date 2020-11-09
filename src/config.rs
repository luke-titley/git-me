//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub server: std::string::String,
    pub private_token: std::string::String,
}

impl Config {
    pub fn file_path() -> std::string::String {
        let base_dirs = directories::BaseDirs::new()
            .expect("Unable to obtain a list of base directories");

        // Return the path
        format!(
            "{}/git_me.yml",
            base_dirs
                .config_dir()
                .to_str()
                .expect("Unable to convert config dir path to unicode")
        )
    }

    pub fn open() -> Self {
        serde_yaml::from_reader(
            std::fs::File::open(&Self::file_path())
                .expect("Unable to create config file"),
        )
        .expect("Unable to read the config from disk")
    }

    pub fn save(&self) {
        serde_yaml::to_writer(
            std::fs::File::create(&Self::file_path())
                .expect("Unable to create config file"),
            self,
        )
        .expect("Unable to write the config to disk");
    }
}

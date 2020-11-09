//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use std::os::unix::fs::PermissionsExt;

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
        let config_file = std::fs::File::create(&Self::file_path())
            .expect("Unable to create config file");

        let metadata = config_file
            .metadata()
            .expect("Unable to obtain meta data for config");
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o600);
        config_file
            .set_permissions(permissions)
            .expect("Unable to set permissions on config file");

        serde_yaml::to_writer(config_file, self)
            .expect("Unable to write the config to disk");
    }
}

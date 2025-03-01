pub mod cd {
    pub mod cd;
    pub mod clean;
    pub mod iface;
    pub mod ls;
    pub mod rm;
    pub mod st;
}
pub mod args;
pub mod bgcd;
pub mod sw;

pub fn data_path() -> std::path::PathBuf {
    let data_path = directories::ProjectDirs::from("com", "github", "ruti").unwrap();
    data_path.data_local_dir().to_path_buf()
}

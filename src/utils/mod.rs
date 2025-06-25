pub mod project;
pub use project::{
    Project, ProjectExistsResult, get_project_by_name, get_store_path, load_projects,
    normalize_path, project_exists, save_projects,
};

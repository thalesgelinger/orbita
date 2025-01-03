use git2::Repository;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::utils::get_orbita_base_dir;

fn clone_repo(repo_path: &str, target_dir: &Path) -> Result<(), git2::Error> {
    let repo_url = format!("https://github.com/{}.git", repo_path);
    println!("Cloning {} into {:?}", repo_url, target_dir);
    Repository::clone(&repo_url, target_dir)?;
    Ok(())
}

fn get_latest_version(repo_path: &str) -> Option<String> {
    let repo_url = format!("https://github.com/{}.git", repo_path);

    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let repo = Repository::clone(&repo_url, temp_dir.path()).ok()?;

    let tags = repo.tag_names(None).ok()?;
    let mut tag_versions: Vec<String> = tags
        .iter()
        .filter_map(|tag| tag.map(String::from))
        .filter(|tag| tag.starts_with('v'))
        .collect();

    tag_versions.sort_by(|a, b| match version_compare::compare(a, b) {
        Ok(version_compare::Cmp::Eq) => std::cmp::Ordering::Equal,
        Ok(version_compare::Cmp::Gt) => std::cmp::Ordering::Greater,
        Ok(version_compare::Cmp::Lt) => std::cmp::Ordering::Less,
        Ok(version_compare::Cmp::Ne)
        | Ok(version_compare::Cmp::Le)
        | Ok(version_compare::Cmp::Ge) => std::cmp::Ordering::Equal,
        Err(_) => std::cmp::Ordering::Equal,
    });

    tag_versions.pop()
}

pub fn add(resource_name: String) {
    println!("Adding resource: {}", resource_name);

    let mut config = Config::load().expect("Failed to load configuration");

    let mut parts = resource_name.split('@');
    let repo_path = parts.next().unwrap();
    let version = parts.next().map(String::from).or_else(|| {
        println!("No version specified, fetching the latest version...");
        Some(get_latest_version(repo_path).unwrap_or_else(|| {
            eprintln!("Failed to fetch the latest version for {}", repo_path);
            std::process::exit(1);
        }))
    });

    config
        .add_dependency(repo_path.to_string(), version.clone(), None)
        .expect("Failed to add dependency");

    let orbita_dir = get_orbita_base_dir();
    let package_dir = orbita_dir
        .join("packages")
        .join(repo_path)
        .join(version.as_deref().unwrap_or("main"));

    if package_dir.exists() {
        println!(
            "Package {} is already installed at {:?}",
            resource_name, package_dir
        );
        return;
    }

    if let Some(parent) = package_dir.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directory");
    }

    match clone_repo(repo_path, &package_dir) {
        Ok(_) => {
            println!(
                "Successfully added {} at version {} to {:?}",
                repo_path,
                version.unwrap_or_else(|| "main".to_string()),
                package_dir
            );
        }
        Err(e) => {
            eprintln!("Failed to add {}: {}", resource_name, e);
        }
    }
}

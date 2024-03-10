use std::path;

use processes;

pub fn resolve_path(path: &path::Path) -> String {
    let canon = path.canonicalize().unwrap();
    let full_path = canon.to_string_lossy().to_string();
    match full_path.strip_prefix("\\\\?\\") {
        Some(stripped) => stripped.to_string(),
        None => full_path
    }
}

pub fn get_processes(needle: &str) -> Result<Vec<path::PathBuf>, processes::ProcessError> {
    let mut collected_proc: Vec<path::PathBuf> = Vec::new();
    for process in processes::all().unwrap() {
        let name = process.base_name()?;
        if !name.to_lowercase().contains(needle) {
            continue;
        }
        let executable = match process.path() {
            Ok(p) => p,
            Err(_) => continue,
        };
        if executable.starts_with("C:\\Windows\\System")
            || collected_proc.iter().any(|p| &executable == p)
        {
            continue;
        };
        collected_proc.push(executable)
    }
    Ok(collected_proc)
}

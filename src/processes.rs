use std::path;

use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub fn system() -> System {
    let refresh = ProcessRefreshKind::new().with_exe(sysinfo::UpdateKind::Always);
    System::new_with_specifics(RefreshKind::new().with_processes(refresh))
}

pub fn resolve_path(path: &path::Path) -> String {
    let canon = path.canonicalize().unwrap();
    let full_path = canon.to_string_lossy().to_string();
    match full_path.strip_prefix("\\\\?\\") {
        Some(stripped) => stripped.to_string(),
        None => full_path,
    }
}

pub fn get_processes(needle: &str) -> Vec<path::PathBuf> {
    let mut sys = system();
    sys.refresh_processes();
    let mut collected_proc: Vec<path::PathBuf> = Vec::new();
    for (_, proc) in sys.processes() {
        let name = proc.name();
        if !name.to_lowercase().contains(needle) {
            continue;
        }
        let executable = match proc.exe() {
            Some(p) => p,
            None => continue,
        };
        if executable.starts_with("C:\\Windows\\System")
            || collected_proc.iter().any(|p| &executable == p)
        {
            continue;
        };
        collected_proc.push(executable.to_path_buf())
    }
    collected_proc
}

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(target_os = "windows")]
    {
        setup_cuda_dlls_windows();
    }
}

#[cfg(target_os = "windows")]
fn setup_cuda_dlls_windows() {
    let mut search_dirs = Vec::new();
    if let Ok(p) = env::var("CUDA_PATH") {
        search_dirs.push(PathBuf::from(&p).join("bin").join("x64"));
        search_dirs.push(PathBuf::from(&p).join("bin"));
    }
    if let Ok(p) = env::var("CUDA_HOME") {
        search_dirs.push(PathBuf::from(&p).join("bin").join("x64"));
        search_dirs.push(PathBuf::from(&p).join("bin"));
    }
    if let Ok(path_var) = env::var("PATH") {
        for entry in env::split_paths(&path_var) {
            if entry.to_string_lossy().to_lowercase().contains("cuda") {
                search_dirs.push(entry);
            }
        }
    }
    let default_toolkit = Path::new(r"C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA");
    if default_toolkit.exists() {
        if let Ok(entries) = fs::read_dir(default_toolkit) {
            for entry in entries.flatten() {
                let bin_x64 = entry.path().join("bin").join("x64");
                if bin_x64.exists() {
                    search_dirs.push(bin_x64);
                }
                let bin = entry.path().join("bin");
                if bin.exists() {
                    search_dirs.push(bin);
                }
            }
        }
    }

    let out_dir = match env::var("OUT_DIR") {
        Ok(d) => PathBuf::from(d),
        Err(_) => return,
    };
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .map(|p| p.to_path_buf());

    let mut dest_dirs = vec![out_dir];
    if let Some(ref p) = profile_dir {
        dest_dirs.push(p.clone());
        dest_dirs.push(p.join("deps"));
    }

    let lib_patterns = [
        ("cublas64.dll", "cublas64_"),
        ("cublasLt64.dll", "cublasLt64_"),
        ("nvrtc64.dll", "nvrtc64_"),
    ];

    for (generic_name, prefix) in lib_patterns {
        for search_dir in &search_dirs {
            if let Ok(entries) = fs::read_dir(search_dir) {
                for entry in entries.flatten() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.starts_with(prefix) && file_name.ends_with(".dll") {
                        let src = entry.path();
                        for dest_dir in &dest_dirs {
                            let _ = fs::create_dir_all(dest_dir);
                            let target = dest_dir.join(generic_name);
                            if !target.exists() {
                                let _ = fs::copy(&src, &target);
                            }
                        }
                    }
                    if file_name.starts_with("nvrtc-builtins") && file_name.ends_with(".dll") {
                        let src = entry.path();
                        for dest_dir in &dest_dirs {
                            let _ = fs::create_dir_all(dest_dir);
                            let target = dest_dir.join(&file_name);
                            if !target.exists() {
                                let _ = fs::copy(&src, &target);
                            }
                        }
                    }
                }
            }
        }
    }
}

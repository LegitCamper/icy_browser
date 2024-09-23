use std::fs::{self, DirEntry};
use std::path::Path;

const PATH: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    // ensure runtime resources exist
    #[cfg(feature = "ultralight")]
    {
        let mut possible_directories = Vec::new();

        let target = Path::new(PATH).join("target");
        let debug_path = target.clone().join("debug");
        let release_path = target.clone().join("release");

        if let Ok(debug) = fs::exists(debug_path.clone()) {
            if debug {
                get_paths(
                    &mut possible_directories,
                    debug_path.join("build").to_str().unwrap().to_string(),
                )
            }
        } else if let Ok(release) = fs::exists(release_path.clone()) {
            if release {
                get_paths(
                    &mut possible_directories,
                    release_path.join("build").to_str().unwrap().to_string(),
                )
            }
        } else {
            panic!("Could not find either debug or release dirs")
        }

        assert!(!possible_directories.is_empty());

        let local_resources = Path::new(PATH).join("resources");

        for path in possible_directories {
            if let Ok(resources) = fs::exists(path.path().join("out/ul-sdk/resources")) {
                if resources {
                    if let Ok(local_resources_exist) = fs::exists(local_resources.clone()) {
                        if local_resources_exist {
                            fs::remove_dir_all(local_resources.clone())
                                .expect("Failed to delete resources dir")
                        }
                    }

                    fs::create_dir(local_resources.clone())
                        .expect("Failed to create resources dir");

                    copy_file(
                        path.path().join("out/ul-sdk/resources").as_path(),
                        local_resources.clone().join("").as_path(),
                        "cacert.pem",
                    )
                    .expect("Failed to copy cacert.pem");
                    copy_file(
                        path.path().join("out/ul-sdk/resources").as_path(),
                        local_resources.clone().join("").as_path(),
                        "icudt67l.dat",
                    )
                    .expect("Failed to copy icudt67l.dat");

                    break;
                }
            } else {
                panic!("The resouce dir entered has not resources")
            }
        }
    }

    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
}

fn copy_file(from: &Path, to: &Path, file_name: &str) -> Result<u64, std::io::Error> {
    fs::copy(from.join(file_name), to.join(file_name))
}

fn get_paths(possible_paths: &mut Vec<fs::DirEntry>, path_str: String) {
    let mut paths: Vec<DirEntry> = fs::read_dir(path_str)
        .expect("Could not read dir")
        .map(|f| f.unwrap())
        .filter(|file| file.path().to_string_lossy().contains("ul-next-sys"))
        .collect();
    // TODO: check if sort working
    paths.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&b.metadata().unwrap().modified().unwrap())
    });
    possible_paths.append(&mut paths);
}

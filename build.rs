use std::env;
use std::fs;
use std::path::Path;


fn get_cargo_target_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}


fn main() {
    let out_dir = get_cargo_target_dir().expect("failed to locate the TARGET folder");
    let config_dir_path = "./config";

    let dst_path = Path::new(&out_dir).join("config");
    let src_path = Path::new(&config_dir_path);


        copy_files_recursive(src_path.as_ref(), &dst_path);
    }

    fn copy_files_recursive(src_path: &Path, dst_path: &Path) {
        println!("{src_path:?} -> {dst_path:?}");
        println!("makeing {dst_path:?}");
        fs::create_dir_all(&dst_path).unwrap();
        if src_path.is_dir() {
            for entry in fs::read_dir(src_path).unwrap() {
                let entry = entry.unwrap();
                let file_name = entry.file_name();
                let src_path = entry.path();
                let dst_path = dst_path.join(file_name);

                if src_path.is_dir() {

                    copy_files_recursive(src_path.as_ref(), dst_path.as_ref());
            } else {
                fs::copy(src_path, dst_path).unwrap();
            }
        }
    }
}
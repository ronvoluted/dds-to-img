use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static PROCESSED_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dds-to-img <directory> [png | webp | avif | gif]");
        return;
    }

    let dir = &args[1];
    let format = if args.len() >= 3 {
        args[2].to_lowercase()
    } else {
        "png".to_string()
    };

    if !["png", "webp", "avif", "gif"].contains(&format.as_str()) {
        println!("Invalid output format. Supported formats: png, webp, avif, gif");
        return;
    }

    let output_dir = format!("{}_{}", dir, format);
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    process_directory(dir, &output_dir, dir, &format);
}

fn process_directory<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    dir: &P,
    output_base_dir: &Q,
    root_dir: &R,
    format: &str,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    process_directory(&path, output_base_dir, root_dir, format);
                } else {
                    process_file(&path, output_base_dir, root_dir, format);
                }
            }
        }
    } else {
        println!("Failed to read directory {:?}", dir.as_ref());
    }
}

fn process_file<P: AsRef<Path>, Q: AsRef<Path>, R: AsRef<Path>>(
    path: &P,
    output_base_dir: &Q,
    root_dir: &R,
    format: &str,
) {
    let path = path.as_ref();
    if let Some(extension) = path.extension() {
        if extension == "dds" {
            let input_file = path.to_str().unwrap();
            let file_stem = path.file_stem().unwrap().to_str().unwrap();

            let relative_path = path.strip_prefix(root_dir).unwrap();
            let parent_dir = relative_path.parent().unwrap();
            let output_subdir = output_base_dir.as_ref().join(parent_dir);
            fs::create_dir_all(&output_subdir).expect("Failed to create output subdirectory");

            let output_bmp_path = output_subdir.join(file_stem).with_extension("BMP");
            let output_file_path = output_subdir.join(file_stem).with_extension(format);

            let texconv_status = Command::new("texconv")
                .args(&[
                    "-ft",
                    "bmp",
                    "-f",
                    "B8G8R8A8_UNORM",
                    "-y",
                    "-o",
                    output_subdir.to_str().unwrap(),
                    input_file,
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .expect("Failed to run texconv");

            if texconv_status.success() {
                let ffmpeg_status = Command::new("ffmpeg")
                    .args(&[
                        "-i",
                        output_bmp_path.to_str().unwrap(),
                        "-update",
                        "1",
                        output_file_path.to_str().unwrap(),
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("Failed to run ffmpeg");

                if ffmpeg_status.success() {
                    fs::remove_file(output_bmp_path).expect("Failed to remove BMP file");
                    let count = PROCESSED_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
                    println!(
                        "{} ({} processed)",
                        output_file_path.to_str().unwrap(),
                        count
                    );
                } else {
                    println!("Failed to process file {:?}", path);
                }
            } else {
                println!("Failed to process file {:?}", path);
            }
        }
    }
}

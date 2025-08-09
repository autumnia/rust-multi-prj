use std::fs;
use std::path::Path;

fn main() {
    // 경로 설정: audio/original -> target/debug/audio/original
    let src_dir = Path::new("audio").join("original");
    let out_dir = Path::new(&std::env::var("OUT_DIR").unwrap())
        .ancestors()
        .nth(3) // OUT_DIR → target/debug/build/.../... → target/debug
        .unwrap()
        .join("audio")
        .join("original");

    // 폴더 생성
    fs::create_dir_all(&out_dir).unwrap();

    // 파일 복사
    for entry in fs::read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let dest_path = out_dir.join(file_name);
        fs::copy(entry.path(), &dest_path).unwrap();
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
}
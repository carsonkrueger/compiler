pub enum FileExt {
    Bin,
    Asm,
    Txt,
}

pub fn file_ext(path: &String) -> Option<FileExt> {
    if path.ends_with(".bin") {
        Some(FileExt::Bin)
    } else if path.ends_with(".asm") {
        Some(FileExt::Asm)
    } else if path.ends_with(".txt") {
        Some(FileExt::Txt)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum FileExt {
    Bin,
    Asm,
    Txt,
}

pub fn file_ext(path: &String) -> Option<FileExt> {
    // if path.ends_with(".bin") {
    //     Some(FileExt::Bin)
    // } else if path.ends_with(".asm") {
    //     Some(FileExt::Asm)
    // } else if path.ends_with(".txt") {
    //     Some(FileExt::Txt)
    // } else {
    //     None
    // }
    if path.len() <= 0 {
        return None;
    }
    match std::path::Path::new(path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
    {
        "bin" => Some(FileExt::Bin),
        "asm" => Some(FileExt::Asm),
        "txt" => Some(FileExt::Txt),
        _ => None,
    }
}

pub fn file_name(path: &String) -> Option<&str> {
    if path.len() <= 0 {
        return None;
    }
    std::path::Path::new(path).file_stem().unwrap().to_str()
}

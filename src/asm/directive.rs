pub enum DirectiveType {
    Byt,
    Int,
    Str,
}

pub enum Directive {
    Byt1(u8),
    Byt4(i32),
    Int1(u8),
    Int4(i32),
    Str(String),
}

impl Directive {
    fn write(&self) {
        let bytes = match &self {
            Directive::Int4(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Int1(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Byt4(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Byt1(i) => i.to_le_bytes().as_slice().to_owned(),
            Directive::Str(s) => s.as_bytes().to_owned(),
        };
    }
}

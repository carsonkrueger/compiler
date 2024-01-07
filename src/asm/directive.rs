pub enum DirectiveType {
    Byt,
    Int,
    Str,
}

pub enum DirectiveValue {
    Byt1(u8),
    Byt4(i32),
    Int1(u8),
    Int4(i32),
    Str(String),
}

pub struct Directive {
    dir_type: DirectiveType,
    value: DirectiveValue,
}

impl Directive {
    fn write(&self) {
        let bytes = match &self.value {
            DirectiveValue::Int4(i) => i.to_le_bytes().as_slice().to_owned(),
            DirectiveValue::Int1(i) => i.to_le_bytes().as_slice().to_owned(),
            DirectiveValue::Byt4(i) => i.to_le_bytes().as_slice().to_owned(),
            DirectiveValue::Byt1(i) => i.to_le_bytes().as_slice().to_owned(),
            DirectiveValue::Str(s) => s.as_bytes().to_owned(),
        };
    }
}

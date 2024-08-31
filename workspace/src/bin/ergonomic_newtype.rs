struct WrappedI32(i32);

impl From<i32> for WrappedI32 {
    fn from(raw: i32) -> Self {
        Self(raw)
    }
}

fn demonstrate() {
    let _ = WrappedI32::from(84);
    let _: WrappedI32 = 84.into();
}

fn main() {
    demonstrate();
}

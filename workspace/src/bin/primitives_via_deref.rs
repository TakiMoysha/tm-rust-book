use std::ops::Deref;

fn take_a_str(_: &str) {}

#[derive(Debug, Clone)]
struct EmailAddress(String);

impl EmailAddress {
    fn new(raw: &str) -> Self {
        Self(raw.into())
    }
}

impl Deref for EmailAddress {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let raw = "email@domain.edu";
    let email = EmailAddress::new(raw);
    take_a_str(email.deref());

    let lower = email.to_lowercase();
    assert_eq!(lower, "email@domain.edu");
    assert_eq!(*raw, *email);
}

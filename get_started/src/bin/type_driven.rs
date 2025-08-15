use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::RangeBounds,
};

use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CreateUserError {
    #[error("invalid email format: {0}")]
    InvalidEmailFormat(String),
    #[error("invalid password: {reason}")]
    InvalidPassword { reason: String },
    #[error("failed to hash password: {0}")]
    PasswordHashingError(#[from] BcryptError),
    #[error("user with email address {email} already exists")]
    UserAlreadyExists { email: String },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmailAddress(String);
#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid email address")]
pub struct EmailAddressError(String);

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl EmailAddress {
    pub fn new(raw_email: &str) -> Result<Self, EmailAddressError> {
        if email_regex().is_match(raw_email) {
            Ok(Self(raw_email.into()))
        } else {
            Err(EmailAddressError(raw_email.to_string()))
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserAlreadyExistsError(EmailAddress);

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

struct User {
    email: String,
    password: String,
}

pub fn create_user(
    email: EmailAddress,
    password: Password,
) -> Result<User, UserAlreadyExistsError> {
    validate_email(&email)?;
    validate_password(&email)?;
    let hashed_password = hash_password(&password)?;

    let validate_email = |email: &str| -> Result<&str, CreateUserError> { Ok(email) };
    let validate_pass = |pass: &str| -> Result<&str, CreateUserError> { Ok(pass) };

    let user = User {
        email: validate_email(email)?.into(),
        password: validate_pass(pass)?.into(),
    };
    Ok(User)
}

#[cfg(test)]
mod email_address_tests {
    use super::*;

    #[test]
    fn should_return_error_by_invalid_email() {
        let email = "invalid-email";
        let pass = "password";
        let result = create_user(EmailAddress(email.to_string()), Password(pass.to_string()));
        let expected_error = CreateUserError::InvalidEmailFormat(email.to_string());
    }
    #[test]
    fn should_return_error_by_user_already_exists() {
        todo!()
    }
}

#[cfg(test)]
mod password_tests {
    use super::*;

    #[test]
    fn should_return_error_by_invalid_password() {
        todo!()
    }
    #[test]
    fn should_return_error_by_hashed_password() {
        todo!()
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn should_return_error_by_user_already_exists() {
        todo!()
    }
}

struct NonEmptyVec<T>(Vec<T>);
impl<T> NonEmptyVec<T> {
    fn pop(&mut self) -> Option<T> {
        if self.0.len() == 1 {
            None
        } else {
            self.0.pop()
        }
    }

    fn last(&self) -> &T {
        self.0.last().unwrap()
    }
}

#[cfg(test)]
mod non_empty_vec_tests {
    use super::*;
}

#[derive(Error, Debug, Copy, Clone, PartialEq)]
#[error("subsecond value must be in the range 0-1, but was {0}")]
pub struct SubsecondError(f64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Subsecond(f64);

impl Subsecond {
    pub fn new(raw: f64) -> Result<Self, SubsecondError> {
        if !(0.0..1.0).contains(&raw) {
            Err(SubsecondError(raw))
        } else {
            Ok(Self(raw))
        }
    }
}

impl Eq for Subsecond {
}

impl PartialEq for Subsecond {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Subsecond {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(ordering) => ordering,
            None => unreachable!(),
        }
    }
}

pub fn main() {}

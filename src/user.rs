#[derive(Debug, PartialEq, Eq)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> User {
        User { name: name.to_string() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user() {
        let user = User::new("John Doe");
        assert_eq!(user.name(), "John Doe");
    }

    #[test]
    fn user_name() {
        let user = User::new("Jane Doe");
        assert_eq!(user.name(), "Jane Doe");
    }
}

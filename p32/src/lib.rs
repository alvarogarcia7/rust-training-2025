struct User {
    name: String,
}

impl User {
    pub(crate) fn new(name: String) -> Self {
        User { name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_constructor_fields() {
        let user = User::new("Name Surname".to_string());

        assert_eq!(user.name, "Name Surname".to_string());
    }
}

pub struct Contact {
    name: String,
    birthday: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    notes: Option<String>,
}

impl Contact {
    pub fn new(
        name: String,
        birthday: String,
        phone: String,
        email: String,
        notes: String,
    ) -> Self {
    }
}

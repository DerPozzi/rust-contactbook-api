#[derive(Debug)]
pub struct Contact {
    pub id: Option<i32>,
    pub name: String,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
}

// impl Contact {
//     pub fn new(
//         name: String,
//         birthday: String,
//         phone: String,
//         email: String,
//         notes: String,
//     ) -> Self {
//     }
// }

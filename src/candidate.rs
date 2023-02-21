use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
}


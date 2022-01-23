#[derive(serde::Serialize)]
pub struct User {
    pub(crate) Id: u16,
    pub(crate) Age: u16,
    pub(crate) First_Name: String,
    pub(crate) Last_Name: String,
    pub(crate) Framework: String,
}

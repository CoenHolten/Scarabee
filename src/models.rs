#[derive(Queryable)]
pub struct User {
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

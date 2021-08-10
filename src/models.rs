use blake2::{Blake2s, Digest};
use rocket::{
    request::{self, FromRequest},
    Request,
};

use super::schema::groups;
use super::schema::users;

#[derive(Queryable, FromForm, Insertable)]
pub struct User {
    pub name: String,
    pub password: String, // this is a hash
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(FromForm, AsChangeset)]
#[table_name = "users"]
pub struct UserEdit {
    pub password: Option<String>, // this is a hash
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl User {
    pub fn hash_password(&mut self) {
        let name = self.name.as_bytes();
        let mut hasher = Blake2s::with_params(&[], name, &[]);
        hasher.update(&self.password);
        self.password = hex::encode(hasher.finalize());
    }
}

impl UserEdit {
    pub fn hash_password(&mut self, name: &str) {
        if let Some(ref pass) = self.password {
            let mut hasher = Blake2s::with_params(&[], name.as_bytes(), &[]);
            hasher.update(pass);
            self.password = Some(hex::encode(hasher.finalize()));
        }
    }
}

pub struct UserAuth(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let username = req.cookies().get_private("name");
        if username.is_none() {
            return request::Outcome::Forward(());
        }
        let username = username.as_ref().unwrap().value();
        request::Outcome::Success(UserAuth(username.to_string()))
    }
}

#[derive(Queryable, FromForm, Insertable)]
pub struct Group {
    pub name: String,
    pub description: String,
    pub commitment: String,
    pub is_commitment: i8, // zero means false
    pub is_concept: i8,    // zero means false
}

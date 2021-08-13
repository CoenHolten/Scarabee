use blake2::{Blake2s, Digest};
use rand::{thread_rng, Rng};
use rocket::{
    request::{self, FromRequest},
    Request,
};

use crate::models::{User, UserLogin};

impl User {
    pub fn hash_password(&mut self) {
        let name = self.name.as_bytes();
        let mut hasher = Blake2s::with_params(&[], name, &[]);
        hasher.update(&self.password);
        self.password = hex::encode(hasher.finalize());
    }
}

impl UserLogin {
    pub fn hash_password(&mut self) {
        let name = self.name.as_bytes();
        let mut hasher = Blake2s::with_params(&[], name, &[]);
        hasher.update(&self.password);
        self.password = hex::encode(hasher.finalize());
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

pub fn random_id(name: &str) -> String {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..10000);
    format!("{}#{:04}", name, num)
}

#[database("3ways_db")]
pub struct DbConn(diesel::MysqlConnection);

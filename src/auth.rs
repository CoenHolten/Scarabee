use blake2::{Blake2s, Digest};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::TextExpressionMethods;
use rand::{thread_rng, Rng};
use rocket::{
    request::{self, FromRequest},
    Request,
};

use crate::models::{Search, User, UserLogin};

impl User {
    pub fn hash_password(&mut self) {
        let name = self.name.as_bytes();
        let len = name.len().min(8);
        let mut hasher = Blake2s::with_params(&[], &name[..len], &[]);
        hasher.update(&self.password);
        self.password = hex::encode(hasher.finalize());
    }
}

impl UserLogin {
    pub fn hash_password(&mut self) {
        let name = self.name.as_bytes();
        let len = name.len().min(8);
        let mut hasher = Blake2s::with_params(&[], &name[..len], &[]);
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

// TODO: Using random ids for unique keys in the db will become slow at one point, and will stop
// working altogether if every number is used. I don't quite know why this is used, and if I remove
// this, adding new commitments will loop infinitely.
pub fn random_id(name: &str) -> String {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..10000);
    format!("{}-{:04}", name, num)
}

#[database("3ways_db")]
pub struct DbConn(diesel::MysqlConnection);

pub enum Column {
    Initiative,
    Support,
    Commitment,
}

pub async fn search_db(conn: DbConn, search: Search, column: Column) -> Vec<String> {
    fn to_filter(option: &Option<String>) -> &str {
        option.as_ref().map(String::as_str).unwrap_or("%")
    }

    conn.run(move |c| {
        use crate::schema::initiatives::dsl::*;
        use crate::schema::supports::dsl::*;
        let combined = initiatives.inner_join(supports);
        let filtered = combined
            .filter(user_name.like(to_filter(&search.support)))
            .filter(commitment_name.like(to_filter(&search.commitment)))
            .filter(initiative_name.like(to_filter(&search.initiative)));

        match column {
            Column::Initiative => filtered
                .select(initiative_name)
                .distinct()
                .load::<String>(c)
                .unwrap(),
            Column::Support => filtered
                .select(user_name)
                .distinct()
                .load::<String>(c)
                .unwrap(),
            Column::Commitment => filtered
                .select(commitment_name)
                .distinct()
                .load::<String>(c)
                .unwrap(),
        }
    })
    .await
}

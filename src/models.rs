use super::schema::*;

#[derive(Queryable, FromForm, Insertable, Identifiable, AsChangeset, Serialize)]
#[primary_key(name)]
pub struct User {
    pub name: String,
    pub password: String, // this is a hash
    pub email: String,
    pub phone: String,
}

#[derive(Queryable, FromForm, Identifiable)]
#[primary_key(name)]
#[table_name = "users"]
pub struct UserLogin {
    pub name: String,
    pub password: String, // this is a hash
}

#[derive(Queryable, FromForm, Insertable, Identifiable, Serialize)]
#[primary_key(name)]
pub struct Commitment {
    pub name: String,
    pub description: String,
    pub is_concept: i8, // zero means false
}

#[derive(FromForm, AsChangeset)]
#[table_name = "commitments"]
pub struct CommitmentEdit {
    pub name: String,
    pub description: Option<String>,
    pub is_concept: Option<i8>, // zero means false
}

#[derive(FromForm, Insertable)]
pub struct InitiativeSupport {
    pub initiative_commitment: String,
    pub initiative_name: String,
}

#[derive(Queryable, FromForm, Insertable, Associations, Identifiable, AsChangeset, Serialize)]
#[belongs_to(Commitment, foreign_key = "commitment")]
#[primary_key(commitment, name)]
pub struct Initiative {
    pub commitment: String,
    pub name: String,
    pub description: String,
    pub user: Option<String>,
}

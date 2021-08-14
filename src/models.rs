use super::schema::*;

#[derive(Queryable, FromForm, Insertable, Identifiable, AsChangeset, Serialize, Clone)]
#[primary_key(name)]
pub struct User {
    #[field(validate = len(..=40))]
    pub name: String,
    pub password: String,
    pub email: String,
    pub phone: String,
}

#[derive(Queryable, FromForm, Identifiable)]
#[primary_key(name)]
#[table_name = "users"]
pub struct UserLogin {
    pub name: String,
    pub password: String,
}

#[derive(Queryable, FromForm, Insertable, Identifiable, Serialize, Clone)]
#[primary_key(name)]
pub struct Commitment {
    #[field(validate = len(..=40))]
    pub name: String,
    pub description: String,
}

#[derive(FromForm, Insertable)]
pub struct InitiativeSupport {
    pub initiative_commitment: String,
    pub initiative_name: String,
}

#[derive(
    Queryable, FromForm, Insertable, Associations, Identifiable, AsChangeset, Serialize, Clone,
)]
#[belongs_to(Commitment, foreign_key = "commitment")]
#[belongs_to(User, foreign_key = "carer")]
#[primary_key(commitment, name)]
pub struct Initiative {
    pub commitment: String,
    #[field(validate = len(..=40))]
    pub name: String,
    pub description: String,
    pub carer: Option<String>,
}

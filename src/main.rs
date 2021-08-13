#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate blake2;
extern crate hex;
#[macro_use]
extern crate serde;

mod auth;
mod connection;
mod models;
mod schema;

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use dotenv::dotenv;
use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::content;

use crate::auth::UserAuth;
use crate::models::Commitment;
use crate::models::Initiative;
use crate::models::InitiativeSupport;
use crate::models::UserLogin;

use {connection::establish_connection, models::User};

#[post("/user_new", data = "<user>")]
fn user_new(mut user: Form<User>, cookies: &CookieJar<'_>) -> Status {
    let conn = establish_connection();

    use schema::users::dsl::*;

    user.hash_password();
    let count = diesel::insert_or_ignore_into(users)
        .values(&*user)
        .execute(&conn)
        .unwrap();

    if count == 1 {
        cookies.add_private(Cookie::new("name", user.name.clone()));
        Status::Created
    } else {
        Status::Conflict
    }
}

#[post("/user_edit", data = "<user>")]
fn user_edit(auth: UserAuth, mut user: Form<User>) -> Status {
    if auth.0 != user.name {
        return Status::Unauthorized;
    }

    let conn = establish_connection();

    user.hash_password();
    diesel::update(&*user).set(&*user).execute(&conn).unwrap();

    Status::Ok
}

#[post("/user_login", data = "<user>")]
fn user_login(mut user: Form<UserLogin>, cookies: &CookieJar<'_>) -> Status {
    let conn = establish_connection();

    use crate::diesel::Identifiable;
    use schema::users::dsl::*;

    let db_user = users
        .find(&user.id())
        .get_result::<User>(&conn)
        .optional()
        .unwrap();

    if let Some(db_user) = db_user {
        user.hash_password();
        if user.password == db_user.password {
            cookies.add_private(Cookie::new("name", user.name.clone()));
            Status::Accepted
        } else {
            Status::NotAcceptable
        }
    } else {
        Status::NotFound
    }
}

#[post("/user_logout")]
fn user_logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("name"));
    Status::Ok
}

#[post("/user/<user_name>")]
fn user(_auth: UserAuth, user_name: &str) -> Result<content::Json<String>, Status> {
    let conn = establish_connection();

    use schema::users::dsl::*;

    let item = users.find(user_name).get_result::<User>(&conn);

    if let Ok(user) = item {
        Ok(content::Json(serde_json::to_string(&user).unwrap()))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/commitment_new", data = "<commitment>")]
fn commitment_new(commitment: Form<Commitment>) -> Status {
    let conn = establish_connection();

    use schema::commitments::dsl::*;

    let count = diesel::insert_or_ignore_into(commitments)
        .values(&*commitment)
        .execute(&conn)
        .unwrap();

    if count == 1 {
        Status::Created
    } else {
        Status::Conflict
    }
}

#[post("/commitment/<commitment_name>")]
fn commitment(_auth: UserAuth, commitment_name: &str) -> Result<content::Json<String>, Status> {
    let conn = establish_connection();

    use schema::commitments::dsl::*;

    let item = commitments
        .find(commitment_name)
        .get_result::<Commitment>(&conn);

    if let Ok(commitment) = item {
        Ok(content::Json(serde_json::to_string(&commitment).unwrap()))
    } else {
        Err(Status::NotFound)
    }
}

#[post("/initiative_new", data = "<initiative>")]
fn initiative_new(auth: UserAuth, initiative: Form<Initiative>) -> Status {
    if initiative.user.is_some() && initiative.user.as_ref().unwrap() != &auth.0 {
        return Status::Unauthorized;
    }

    let conn = establish_connection();

    use schema::initiatives::dsl::*;

    let count = diesel::insert_or_ignore_into(initiatives)
        .values(&*initiative)
        .execute(&conn)
        .unwrap();

    if count == 1 {
        let support = InitiativeSupport {
            initiative_commitment: initiative.commitment.clone(),
            initiative_name: initiative.name.clone(),
        };

        initiative_support_add(auth, Form::from(support));

        Status::Created
    } else {
        Status::Conflict
    }
}

#[post("/initiative/<commitment_name>/<initiative_name>")]
fn initiative(
    _auth: UserAuth,
    commitment_name: &str,
    initiative_name: &str,
) -> Result<content::Json<String>, Status> {
    let conn = establish_connection();

    use schema::initiatives::dsl::*;

    let item = initiatives
        .find((commitment_name, initiative_name))
        .get_result::<Initiative>(&conn);

    if let Ok(initiative) = item {
        Ok(content::Json(serde_json::to_string(&initiative).unwrap()))
    } else {
        Err(Status::NotFound)
    }
}

// #[post("/initiative_edit", data = "<initiative>")]
// fn initiative_edit(auth: UserAuth, initiative: Form<Initiative>) -> Status {
//     let conn = establish_connection();

//     let count = diesel::update(&*initiative)
//         .set(&*initiative)
//         .execute(&conn)
//         .unwrap();

//     if count == 1 {
//         Status::Ok
//     } else {
//         Status::NotFound
//     }
// }

#[post("/initiative_support_add", data = "<support>")]
fn initiative_support_add(auth: UserAuth, support: Form<InitiativeSupport>) {
    let conn = establish_connection();

    use schema::initiative_supports::dsl::*;

    diesel::insert_or_ignore_into(initiative_supports)
        .values((user.eq(&auth.0), &*support))
        .execute(&conn)
        .unwrap();
}

#[post("/initiative_support_remove", data = "<support>")]
fn initiative_support_remove(auth: UserAuth, support: Form<InitiativeSupport>) {
    let conn = establish_connection();

    use schema::initiative_supports::dsl::*;

    diesel::delete(initiative_supports)
        .filter(user.eq(&auth.0))
        .filter(initiative_commitment.eq(&support.initiative_commitment))
        .filter(initiative_name.eq(&support.initiative_name))
        .execute(&conn)
        .unwrap();
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // doesn't matter if there is no env file

    rocket::build().mount(
        "/",
        routes![
            user_new,
            user_edit,
            user_login,
            user_logout,
            user,
            commitment_new,
            commitment,
            initiative_new,
            initiative,
            initiative_support_add,
            initiative_support_remove
        ],
    )
}

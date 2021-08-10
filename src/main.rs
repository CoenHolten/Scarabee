#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate blake2;
extern crate hex;

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

use crate::models::Group;
use crate::models::UserEdit;

use {
    connection::establish_connection,
    models::{User, UserAuth},
};

#[post("/new_user", data = "<user>")]
fn new_user(mut user: Form<User>, cookies: &CookieJar<'_>) -> Status {
    let conn = establish_connection();

    use schema::users::dsl::*;

    let db_user = users.filter(name.eq(&user.name)).execute(&conn).unwrap();

    if db_user == 0 {
        user.hash_password();
        diesel::insert_into(users)
            .values(&*user)
            .execute(&conn)
            .unwrap();

        cookies.add_private(Cookie::new("name", user.name.clone()));
        Status::Created
    } else {
        Status::Conflict
    }
}

#[post("/edit_user", data = "<user_edit>")]
fn edit_user(user: UserAuth, mut user_edit: Form<UserEdit>) -> Status {
    let conn = establish_connection();

    use schema::users::dsl::*;

    user_edit.hash_password(&user.0);
    diesel::update(users)
        .filter(name.eq(&user.0))
        .set(&*user_edit)
        .execute(&conn)
        .unwrap();

    Status::Ok
}

#[post("/login", data = "<user>")]
fn login(mut user: Form<User>, cookies: &CookieJar<'_>) -> Status {
    let conn = establish_connection();

    use schema::users::dsl::*;

    let db_user = users
        .filter(name.eq(&user.name))
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

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("name"));
    Status::Ok
}

#[post("/new_group", data = "<group>")]
fn new_group(user: UserAuth, group: Form<Group>) -> Status {
    let conn = establish_connection();

    use schema::groups::dsl::*;

    let db_group = groups.filter(name.eq(&group.name)).execute(&conn).unwrap();

    if db_group == 0 {
        diesel::insert_into(groups)
            .values(&*group)
            .execute(&conn)
            .unwrap();
        Status::Created
    } else {
        Status::Conflict
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // doesn't matter if there is no env file

    rocket::build().mount("/", routes![new_user, edit_user, login, logout, new_group])
}

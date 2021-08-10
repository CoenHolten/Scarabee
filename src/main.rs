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
use crate::models::GroupAdoption;
use crate::models::UserEdit;
use crate::models::UserRelation;

use {
    connection::establish_connection,
    models::{User, UserAuth},
};

#[post("/new_user", data = "<user>")]
fn new_user(mut user: Form<User>, cookies: &CookieJar<'_>) -> Status {
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

    let count = diesel::insert_or_ignore_into(groups)
        .values(&*group)
        .execute(&conn)
        .unwrap();

    if count == 1 {
        Status::Created
    } else {
        Status::Conflict
    }
}

#[post("/user_relation_set", data = "<relation>")]
fn user_relation_set(user_auth: UserAuth, mut relation: Form<UserRelation>) {
    relation.user = Some(user_auth.0.clone());

    let conn = establish_connection();

    use schema::user_relations::dsl::*;

    let count = diesel::update(user_relations)
        .filter(user.eq(&user_auth.0))
        .filter(group.eq(&relation.group))
        .set(&*relation)
        .execute(&conn)
        .unwrap();

    if count == 0 {
        diesel::insert_into(user_relations)
            .values(&*relation)
            .execute(&conn)
            .unwrap();
    }
}

#[post("/group_relation_add", data = "<relation>")]
fn group_adoption_add(user_auth: UserAuth, mut relation: Form<GroupAdoption>) {
    relation.user = Some(user_auth.0);

    let conn = establish_connection();

    use schema::group_adoptions::dsl::*;

    diesel::insert_or_ignore_into(group_adoptions)
        .values(&*relation)
        .execute(&conn)
        .unwrap();
}

#[post("/group_relation_remove", data = "<relation>")]
fn group_adoption_remove(user_auth: UserAuth, relation: Form<GroupAdoption>) {
    let conn = establish_connection();

    use schema::group_adoptions::dsl::*;

    diesel::delete(group_adoptions)
        .filter(user.eq(user_auth.0))
        .filter(parent_group.eq(&relation.parent_group))
        .filter(child_group.eq(&relation.child_group))
        .execute(&conn)
        .unwrap();
}

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // doesn't matter if there is no env file

    rocket::build().mount("/", routes![new_user, edit_user, login, logout, new_group])
}

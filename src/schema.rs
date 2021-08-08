table! {
    group (name) {
        name -> Varchar,
        description -> Longtext,
        commitment -> Nullable<Longtext>,
        is_commitment -> Tinyint,
        concept -> Nullable<Longtext>,
    }
}

table! {
    group_adoptions (user, child_group, parent_group) {
        user -> Varchar,
        parent_group -> Varchar,
        child_group -> Varchar,
    }
}

table! {
    user (name) {
        name -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

table! {
    user_relation (user, group) {
        user -> Varchar,
        group -> Varchar,
        accepts_concept -> Tinyint,
        is_adoption -> Tinyint,
    }
}

joinable!(group_adoptions -> user (user));
joinable!(user_relation -> group (group));
joinable!(user_relation -> user (user));

allow_tables_to_appear_in_same_query!(group, group_adoptions, user, user_relation,);

table! {
    group_adoptions (user, child_group, parent_group) {
        user -> Varchar,
        parent_group -> Varchar,
        child_group -> Varchar,
    }
}

table! {
    groups (name) {
        name -> Varchar,
        description -> Longtext,
        commitment -> Nullable<Longtext>,
        is_commitment -> Tinyint,
        is_concept -> Nullable<Tinyint>,
    }
}

table! {
    user_relations (user, group) {
        user -> Varchar,
        group -> Varchar,
        is_adoption -> Tinyint,
        is_support -> Tinyint,
    }
}

table! {
    users (name) {
        name -> Varchar,
        password -> Varchar,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

joinable!(group_adoptions -> users (user));
joinable!(user_relations -> groups (group));
joinable!(user_relations -> users (user));

allow_tables_to_appear_in_same_query!(
    group_adoptions,
    groups,
    user_relations,
    users,
);

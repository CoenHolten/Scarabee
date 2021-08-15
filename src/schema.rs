table! {
    commitments (name) {
        name -> Varchar,
        description -> Text,
    }
}

table! {
    initiatives (name) {
        name -> Varchar,
        commitment_name -> Varchar,
        description -> Text,
    }
}

table! {
    supports (user_name, initiative_name) {
        user_name -> Varchar,
        initiative_name -> Varchar,
        adopt_since -> Nullable<Datetime>,
    }
}

table! {
    users (name) {
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
        phone -> Varchar,
    }
}

joinable!(initiatives -> commitments (commitment_name));
joinable!(supports -> initiatives (initiative_name));
joinable!(supports -> users (user_name));

allow_tables_to_appear_in_same_query!(
    commitments,
    initiatives,
    supports,
    users,
);

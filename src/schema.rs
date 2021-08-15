table! {
    commitments (name) {
        name -> Varchar,
        description -> Text,
    }
}

table! {
    initiatives (commitment, name) {
        commitment -> Varchar,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    supports (user, initiative_commitment, initiative_name) {
        user -> Varchar,
        initiative_commitment -> Varchar,
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

joinable!(initiatives -> commitments (commitment));
joinable!(supports -> users (user));

allow_tables_to_appear_in_same_query!(commitments, initiatives, supports, users,);

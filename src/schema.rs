table! {
    commitment_supports (user, commitment) {
        user -> Varchar,
        commitment -> Varchar,
    }
}

table! {
    commitments (name) {
        name -> Varchar,
        description -> Text,
    }
}

table! {
    initiative_supports (user, initiative_commitment, initiative_name) {
        user -> Varchar,
        initiative_commitment -> Varchar,
        initiative_name -> Varchar,
    }
}

table! {
    initiatives (commitment, name) {
        commitment -> Varchar,
        name -> Varchar,
        description -> Text,
        carer -> Nullable<Varchar>,
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

joinable!(commitment_supports -> commitments (commitment));
joinable!(commitment_supports -> users (user));
joinable!(initiative_supports -> users (user));
joinable!(initiatives -> commitments (commitment));
joinable!(initiatives -> users (carer));

allow_tables_to_appear_in_same_query!(
    commitment_supports,
    commitments,
    initiative_supports,
    initiatives,
    users,
);

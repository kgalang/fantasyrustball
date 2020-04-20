table! {
    league_rulesets (id) {
        id -> Uuid,
        points_per_mile -> Int4,
        league_id -> Uuid,
    }
}

table! {
    leagues (id) {
        id -> Uuid,
        name -> Varchar,
        start -> Timestamp,
        rounds -> Int4,
        current_round -> Int4,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(league_rulesets -> leagues (league_id));

allow_tables_to_appear_in_same_query!(
    league_rulesets,
    leagues,
    users,
);

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
    team_owners (id) {
        id -> Uuid,
        user_id -> Uuid,
        team_id -> Uuid,
    }
}

table! {
    team_players (id) {
        id -> Uuid,
        user_id -> Uuid,
        team_id -> Uuid,
    }
}

table! {
    teams (id) {
        id -> Uuid,
        name -> Varchar,
        wins -> Int4,
        losses -> Int4,
        ties -> Int4,
        league_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(league_rulesets -> leagues (league_id));
joinable!(team_owners -> teams (team_id));
joinable!(team_owners -> users (user_id));
joinable!(team_players -> teams (team_id));
joinable!(team_players -> users (user_id));
joinable!(teams -> leagues (league_id));

allow_tables_to_appear_in_same_query!(
    league_rulesets,
    leagues,
    team_owners,
    team_players,
    teams,
    users,
);

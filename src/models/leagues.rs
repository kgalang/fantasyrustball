use crate::schema::{league_rulesets, leagues};
use chrono::NaiveDateTime;
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct League {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewLeague {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "leagues"]
pub struct UpdateLeague {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, Insertable,
)]
#[table_name = "league_rulesets"]
pub struct Ruleset {
    pub id: Uuid,
    pub points_per_mile: i32,
    pub league_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRuleset {
    pub id: Uuid,
    pub points_per_mile: i32,
    pub league_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "league_rulesets"]
pub struct UpdateRuleset {
    pub points_per_mile: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Queryable)]
pub struct LeagueDetails {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
    pub points_per_mile: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LeaguesResponse(pub Vec<LeagueDetails>);

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateLeagueRequest {
    pub name: String,
    pub start: String,
    pub rounds: i32,
    pub points_per_mile: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateLeagueRequest {
    pub name: String,
    pub start: String,
    pub rounds: i32,
    pub points_per_mile: i32,
}

impl From<Vec<LeagueDetails>> for LeaguesResponse {
    fn from(leagues: Vec<LeagueDetails>) -> Self {
        LeaguesResponse(leagues.into_par_iter().map(|league| league).collect())
    }
}

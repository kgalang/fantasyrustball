use crate::schema::{league_rulesets, leagues};
use chrono::NaiveDateTime;
use uuid::Uuid;

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

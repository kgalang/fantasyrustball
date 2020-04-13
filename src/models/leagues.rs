use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(
  Debug, PartialEq,
  Serialize, Deserialize,
  Queryable, Identifiable, Insertable,
)]
#[table_name = "leagues"]
pub struct League {
  pub id: Uuid,
  pub name: String,
  pub start: NaiveDateTime,
  pub rounds: u8,
  pub current_round: u8,
}

#[derive(
  Debug,
  Serialize, Deserialize,
  Queryable, Identifiable, Insertable,
)]
#[table_name = "league_rulesets"]
pub struct Ruleset {
  pub id: Uuid,
  pub league_id: Uuid,
  pub points_per_mile: i32,
}



use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::schema::{leagues, league_rulesets};
use diesel::prelude::*;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
pub struct League {
  pub id: Uuid,
  pub name: String,
  pub start: NaiveDateTime,
  pub rounds: i32,
  pub current_round: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "league_rulesets"]
pub struct Ruleset {
  pub id: Uuid,
  pub points_per_mile: i32,
  pub league_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct LeagueDetails {
  pub id: Uuid,
  pub name: String,
  pub start: NaiveDateTime,
  pub rounds: i32,
  pub current_round: Option<i32>,
  pub points_per_mile: i32,
}

type LeagueDetailsColumns = (
  leagues::id,
  leagues::name,
  leagues::start,
  leagues::rounds,
  leagues::current_round,
  league_rulesets::points_per_mile,
);

pub const LEAGUE_DETAILS_COLUMNS: LeagueDetailsColumns = (
  leagues::id,
  leagues::name,
  leagues::start,
  leagues::rounds,
  leagues::current_round,
  league_rulesets::points_per_mile,
);

pub fn get_all_details(pool: &PoolType) -> Result<Vec<LeagueDetails>, ApiError> {
  let conn = pool.get()?;
  let query = leagues::table.inner_join(league_rulesets::table)
    .select(LEAGUE_DETAILS_COLUMNS);

  let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
  println!("debug statement: {:?}\n", debug);
  let all_leagues = query.load(&conn)?;
  println!("{:?}\n", all_leagues);

  Ok(all_leagues)
}


#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::tests::helpers::tests::get_pool;

  pub fn get_all_leagues_with_details() -> Result<Vec<LeagueDetails>, ApiError> {
    let pool = get_pool();
    get_all_details(&pool)
  }

  #[test]
  fn it_gets_all_leagues_with_details() {
    let leagues = get_all_leagues_with_details();
    assert!(leagues.is_ok());
  }
}
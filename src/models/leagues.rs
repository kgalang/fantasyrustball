
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::database::PoolType;
use crate::errors::ApiError;
use crate::schema::leagues;
use diesel::prelude::*;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
pub struct League {
  pub id: Uuid,
  pub name: String,
  pub start: NaiveDateTime,
  pub rounds: i32,
  pub current_round: Option<i32>,
}

pub fn get_all(pool: &PoolType) -> Result<Vec<League>, ApiError> {
  let conn = pool.get()?;
  let all_leagues = leagues::table.load(&conn)?;

  Ok(all_leagues)
}


#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::tests::helpers::tests::get_pool;

  pub fn get_all_leagues() -> Result<Vec<League>, ApiError> {
    let pool = get_pool();
    get_all(&pool)
  }

  #[test]
  fn it_gets_all_leagues() {
    let leagues = get_all_leagues();
    assert!(leagues.is_ok());
  }
}
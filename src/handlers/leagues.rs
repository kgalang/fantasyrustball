use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::respond_json;
use crate::models::leagues::{find_with_details, get_all_details};
use actix_web::web::{block, Data, Json, Path};
use chrono::NaiveDateTime;
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Queryable)]
pub struct LeagueDetails {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: Option<i32>,
    pub points_per_mile: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct LeaguesResponse(pub Vec<LeagueDetails>);

pub async fn get_league(
    league_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<Json<LeagueDetails>, ApiError> {
    let league: LeagueDetails = block(move || find_with_details(&pool, *league_id)).await?;
    respond_json(league)
}

pub async fn get_leagues(pool: Data<PoolType>) -> Result<Json<LeaguesResponse>, ApiError> {
    let leagues: LeaguesResponse = block(move || get_all_details(&pool)).await?;
    respond_json(leagues)
}

impl From<Vec<LeagueDetails>> for LeaguesResponse {
    fn from(leagues: Vec<LeagueDetails>) -> Self {
        LeaguesResponse(leagues.into_par_iter().map(|league| league).collect())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::{get_data_pool, get_pool};

    pub fn get_all_leagues() -> LeaguesResponse {
        let pool = get_pool();
        get_all_details(&pool).unwrap()
    }

    pub fn get_first_leagues_id() -> Uuid {
        get_all_leagues().0[0].id
    }

    #[actix_rt::test]
    async fn it_gets_all_leagues() {
        let response = get_leagues(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_leagues().0[0]);
    }

    #[actix_rt::test]
    async fn it_gets_a_league() {
        let first_league = &get_all_leagues().0[0];
        let league_id: Path<Uuid> = get_first_leagues_id().into();
        let response = get_league(league_id, get_data_pool()).await.unwrap();
        assert_eq!(response.into_inner(), *first_league);
    }

    #[actix_rt::test]
    async fn it_doesnt_find_a_league() {
        let uuid = Uuid::new_v4();
        let league_id: Path<Uuid> = uuid.into();
        let response = get_league(league_id, get_data_pool()).await;
        let expected_error = ApiError::NotFound(format!("League {} not found", uuid.to_string()));
        assert!(response.is_err());
        assert_eq!(response.unwrap_err(), expected_error);
    }
}

use crate::database::PoolType;
use crate::errors::ApiError;
use crate::helpers::{respond_json, respond_ok};
use crate::models::leagues::{
    create, delete, find_with_details, get_all_details, update, League, NewLeague, NewRuleset,
    Ruleset, UpdateLeague, UpdateRuleset,
};
use actix_web::web::{block, Data, HttpResponse, Json, Path};
use chrono::NaiveDateTime;
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

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

pub async fn create_league(
    pool: Data<PoolType>,
    params: Json<CreateLeagueRequest>,
) -> Result<Json<LeagueDetails>, ApiError> {
    let new_league_id = Uuid::new_v4();
    let date_time_str = [params.start.clone(), " 00:00:00".to_string()].concat();
    let league_start = NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S")?;

    let new_league: League = NewLeague {
        id: new_league_id,
        name: params.name.to_string(),
        start: league_start,
        rounds: params.rounds.into(),
        current_round: 0,
    }
    .into();

    let new_ruleset_id = Uuid::new_v4();
    let new_ruleset: Ruleset = NewRuleset {
        id: new_ruleset_id,
        league_id: new_league_id,
        points_per_mile: params.points_per_mile.into(),
    }
    .into();

    let league = block(move || create(&pool, &new_league, &new_ruleset)).await?;
    respond_json(league)
}

pub async fn update_league(
    league_id: Path<Uuid>,
    pool: Data<PoolType>,
    params: Json<UpdateLeagueRequest>,
) -> Result<Json<LeagueDetails>, ApiError> {
    let date_time_str = [params.start.clone(), " 00:00:00".to_string()].concat();
    let league_start = NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S")?;

    let update_league = UpdateLeague {
        id: *league_id,
        name: params.name.to_string(),
        start: league_start,
        rounds: params.rounds.into(),
        current_round: 0,
    };

    let update_ruleset = UpdateRuleset {
        points_per_mile: params.points_per_mile.into(),
    };

    let league = block(move || update(&pool, &update_league, &update_ruleset)).await?;
    respond_json(league)
}

pub async fn delete_league(
    league_id: Path<Uuid>,
    pool: Data<PoolType>,
) -> Result<HttpResponse, ApiError> {
    block(move || delete(&pool, *league_id)).await?;
    respond_ok()
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

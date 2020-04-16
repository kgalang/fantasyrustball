use crate::database::PoolType;
use crate::errors::ApiError;
use crate::models::leagues::{get_all_details};
use crate::helpers::{respond_json};
use actix_web::web::{block, Data, Json};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use chrono::{NaiveDateTime};


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

    #[actix_rt::test]
    async fn it_gets_all_tournaments() {
        let response = get_leagues(get_data_pool()).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().0[0], get_all_leagues().0[0]);
    }
}
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
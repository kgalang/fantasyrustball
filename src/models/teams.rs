use crate::models::users::User;
use crate::schema::{team_owners, team_players, teams};
use rayon::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub league_id: Uuid,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset, Insertable)]
#[belongs_to(Team)]
#[table_name = "team_owners"]
pub struct Owner {
    pub id: Uuid,
    pub user_id: Uuid,
    pub team_id: Uuid,
}

#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset, Insertable)]
#[belongs_to(Team)]
#[table_name = "team_players"]
pub struct Player {
    pub id: Uuid,
    pub user_id: Uuid,
    pub team_id: Uuid,
}

#[derive(Debug)]
pub struct TeamDetails {
    pub id: Uuid,
    pub league_id: Uuid,
    pub name: String,
    pub wins: i32,
    pub losses: i32,
    pub ties: i32,
    pub players: Option<Vec<Player>>,
    pub owners: Option<Vec<Owner>>,
}

pub struct TeamsResponse(pub Vec<Team>);

impl From<Vec<Team>> for TeamsResponse {
    fn from(team: Vec<Team>) -> Self {
        TeamsResponse(team.into_par_iter().map(|team| team).collect())
    }
}

use crate::database::PoolType;
use crate::errors::ApiError;
use crate::models::teams::*;
use crate::schema::{team_owners, team_players, teams};
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_all(pool: &PoolType) -> Result<TeamsResponse, ApiError> {
    let conn = pool.get()?;
    let all_teams = teams::table.load(&conn)?;

    Ok(all_teams.into())
}

pub fn find(pool: &PoolType, team_id: Uuid) -> Result<TeamDetails, ApiError> {
    let conn = pool.get()?;

    let team: Team = teams::table
        .select(teams::all_columns)
        .filter(teams::id.eq(team_id.clone()))
        .first(&conn)?;

    let found_owners: Option<Vec<Owner>> = team_owners::table
        .select(team_owners::all_columns)
        .filter(team_owners::team_id.eq(team_id))
        .load(&conn)
        .optional()?;

    let found_players: Option<Vec<Player>> = team_players::table
        .select(team_players::all_columns)
        .filter(team_players::team_id.eq(team_id))
        .load(&conn)
        .optional()?;

    let found_team_details = TeamDetails {
        id: team.id.into(),
        league_id: team.league_id,
        name: team.name,
        wins: team.wins,
        losses: team.losses,
        ties: team.ties,
        players: found_players,
        owners: found_owners,
    };

    Ok(found_team_details)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_teams() -> Result<TeamsResponse, ApiError> {
        let pool = get_pool();
        get_all(&pool)
    }

    #[test]
    fn it_finds_team() {
        let teams = get_all_teams().unwrap();
        let team = &teams.0[0];
        let found_team = find(&get_pool(), team.id).unwrap();
        assert_eq!(team.id, found_team.id);
    }

    #[test]
    fn it_gets_all_teams() {
        let teams = get_all_teams();
        assert!(teams.is_ok());
    }
}

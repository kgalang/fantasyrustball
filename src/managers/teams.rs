use crate::database::PoolType;
use crate::errors::ApiError;
use crate::models::teams::*;
use crate::schema::{leagues, team_owners, team_players, teams};
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_all(pool: &PoolType) -> Result<TeamsResponse, ApiError> {
    let conn = pool.get()?;
    let all_teams = teams::table.load(&conn)?;

    Ok(all_teams.into())
}

pub fn get_all_in_league(pool: &PoolType, league_id: Uuid) -> Result<TeamsResponse, ApiError> {
    let conn = pool.get()?;
    let teams_found = teams::table
        .select(teams::all_columns)
        .filter(teams::league_id.eq(league_id.clone()))
        .load(&conn)?;

    Ok(teams_found.into())
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

pub fn create(
    pool: &PoolType,
    new_team: &Team,
    new_players: Option<&Vec<Player>>,
    new_owners: Option<&Vec<Owner>>,
) -> Result<TeamDetails, ApiError> {
    let conn = pool.get()?;

    new_team.insert_into(teams::table).execute(&conn)?;

    let mut inserted_players = None;
    if let Some(new_players) = new_players {
        inserted_players = diesel::insert_into(team_players::table)
            .values(new_players)
            .get_results(&conn)
            .optional()?;
    }

    let mut inserted_owners = None;
    if let Some(new_owners) = new_owners {
        inserted_owners = diesel::insert_into(team_owners::table)
            .values(new_owners)
            .get_results(&conn)
            .optional()?;
    }

    let created = TeamDetails {
        id: new_team.id,
        league_id: new_team.league_id,
        name: new_team.name.to_string(),
        wins: new_team.wins,
        losses: new_team.losses,
        ties: new_team.ties,
        players: inserted_players,
        owners: inserted_owners,
    };

    Ok(created)
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

    pub fn create_tournament_with_owner_and_player() -> Result<TeamDetails, ApiError> {
        let pool = get_pool();
        // Uuid nil is all 0
        let league_id = Uuid::nil();
        let user_id = Uuid::nil();
        let team_id = Uuid::new_v4();

        let team = Team {
            id: team_id,
            name: "Test Team".to_string(),
            wins: 0,
            losses: 0,
            ties: 0,
            league_id: league_id,
        };

        let player = Player {
            id: Uuid::new_v4(),
            team_id: team_id,
            user_id: user_id,
        };

        let owner = Owner {
            id: Uuid::new_v4(),
            team_id: team_id,
            user_id: user_id,
        };

        create(&pool, &team, Some(&vec![player]), Some(&vec![owner]))
    }
    #[test]
    fn it_creates_team_with_owners_and_players() {
        let created = create_tournament_with_owner_and_player();
        assert!(created.is_ok());
    }

    pub fn create_tournament_without_owner_and_player() -> Result<TeamDetails, ApiError> {
        let pool = get_pool();
        // Uuid nil is all 0
        let league_id = Uuid::nil();
        let team_id = Uuid::new_v4();

        let team = Team {
            id: team_id,
            name: "Test Team".to_string(),
            wins: 0,
            losses: 0,
            ties: 0,
            league_id: league_id,
        };

        create(&pool, &team, None, None)
    }
    #[test]
    fn it_creates_team_alone() {
        let created = create_tournament_without_owner_and_player();
        assert!(created.is_ok());
    }
}

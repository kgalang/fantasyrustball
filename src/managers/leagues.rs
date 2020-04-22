use crate::database::PoolType;
use crate::errors::ApiError;
use crate::models::leagues::*;
use crate::schema::{league_rulesets, leagues};
use diesel::prelude::*;
use uuid::Uuid;

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

pub fn find_with_details(pool: &PoolType, league_id: Uuid) -> Result<LeagueDetails, ApiError> {
    use crate::schema::leagues::dsl::{id, leagues};

    let not_found = format!("League {} not found", league_id);
    let conn = pool.get()?;
    let query = leagues
        .filter(id.eq(league_id))
        .inner_join(league_rulesets::table)
        .select(LEAGUE_DETAILS_COLUMNS);
    let league = query.first(&conn);

    if league.is_ok() {
        Ok(league?)
    } else {
        Err(ApiError::NotFound(not_found))
    }
}

pub fn get_all_details(pool: &PoolType) -> Result<LeaguesResponse, ApiError> {
    let conn = pool.get()?;
    let query = leagues::table
        .inner_join(league_rulesets::table)
        .select(LEAGUE_DETAILS_COLUMNS);
    let all_leagues = query.load(&conn)?;

    Ok(all_leagues.into())
}

pub fn create(
    pool: &PoolType,
    new_league: &League,
    new_ruleset: &Ruleset,
) -> Result<LeagueDetails, ApiError> {
    use crate::schema::league_rulesets::dsl::league_rulesets;
    use crate::schema::leagues::dsl::leagues;

    let conn = pool.get()?;

    diesel::insert_into(leagues)
        .values(new_league)
        .execute(&conn)?;

    diesel::insert_into(league_rulesets)
        .values(new_ruleset)
        .execute(&conn)?;

    let created = LeagueDetails {
        id: new_league.id,
        name: new_league.name.clone(),
        start: new_league.start,
        rounds: new_league.rounds,
        current_round: new_league.current_round,
        points_per_mile: new_ruleset.points_per_mile,
    };
    Ok(created.clone().into())
}

pub fn update(
    pool: &PoolType,
    update_league: &UpdateLeague,
    update_ruleset: &UpdateRuleset,
) -> Result<LeagueDetails, ApiError> {
    let conn = pool.get()?;

    let related_ruleset: Ruleset = league_rulesets::table
        .select(league_rulesets::all_columns)
        .filter(league_rulesets::league_id.eq(update_league.id.clone()))
        .first(&conn)?;

    // Update ruleset
    let ruleset_target = league_rulesets::table.filter(league_rulesets::id.eq(related_ruleset.id));
    diesel::update(ruleset_target)
        .set(update_ruleset)
        .execute(&conn)?;

    // Update league
    let league_target = leagues::table.filter(leagues::id.eq(update_league.id));
    diesel::update(league_target)
        .set(update_league)
        .execute(&conn)?;

    // Create return obj
    let updated = LeagueDetails {
        id: update_league.id,
        name: update_league.name.clone(),
        start: update_league.start,
        rounds: update_league.rounds,
        current_round: update_league.current_round,
        points_per_mile: update_ruleset.points_per_mile,
    };

    Ok(updated.clone().into())
}

pub fn delete(pool: &PoolType, league_id: Uuid) -> Result<(), ApiError> {
    let conn = pool.get()?;

    let ruleset_to_delete =
        league_rulesets::table.filter(league_rulesets::league_id.eq(league_id.clone()));
    diesel::delete(ruleset_to_delete).execute(&conn)?;

    let league_to_delete = leagues::table.filter(leagues::id.eq(league_id.clone()));
    diesel::delete(league_to_delete).execute(&conn)?;

    Ok(())
}

impl From<NewLeague> for League {
    fn from(league: NewLeague) -> Self {
        League {
            id: league.id,
            name: league.name,
            start: league.start,
            rounds: league.rounds,
            current_round: league.current_round,
        }
    }
}

impl From<NewRuleset> for Ruleset {
    fn from(ruleset: NewRuleset) -> Self {
        Ruleset {
            id: ruleset.id,
            league_id: ruleset.league_id,
            points_per_mile: ruleset.points_per_mile,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::tests::helpers::tests::get_pool;

    pub fn get_all_leagues_with_details() -> Result<LeaguesResponse, ApiError> {
        let pool = get_pool();
        get_all_details(&pool)
    }

    #[test]
    fn it_finds_league_with_details() {
        let leagues = get_all_leagues_with_details().unwrap();
        let league = &leagues.0[0];
        let found_league = find_with_details(&get_pool(), league.id).unwrap();
        assert_eq!(league, &found_league);
    }

    #[test]
    fn it_gets_all_leagues_with_details() {
        let leagues = get_all_leagues_with_details();
        assert!(leagues.is_ok());
    }
}

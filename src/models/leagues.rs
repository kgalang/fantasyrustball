use crate::database::PoolType;
use crate::errors::ApiError;
use crate::handlers::leagues::{LeagueDetails, LeaguesResponse};
use crate::schema::{league_rulesets, leagues};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
pub struct League {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewLeague {
    pub id: Uuid,
    pub name: String,
    pub start: NaiveDateTime,
    pub rounds: i32,
    pub current_round: i32,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, Insertable,
)]
#[table_name = "league_rulesets"]
pub struct Ruleset {
    pub id: Uuid,
    pub points_per_mile: i32,
    pub league_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRuleset {
    pub id: Uuid,
    pub points_per_mile: i32,
    pub league_id: Uuid,
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

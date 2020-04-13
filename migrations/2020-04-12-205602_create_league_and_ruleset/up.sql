CREATE TABLE leagues (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(100) NOT NULL,
  start TIMESTAMP NOT NULL,
  rounds INT NOT NULL,
  current_round INT
);

CREATE TABLE league_rulesets (
    id UUID NOT NULL PRIMARY KEY,
    points_per_mile INT NOT NULL,
    league_id UUID NOT NULL REFERENCES leagues (id) ON DELETE CASCADE
);
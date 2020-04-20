CREATE TABLE leagues (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(100) NOT NULL,
  start TIMESTAMP NOT NULL,
  rounds INT NOT NULL,
  current_round INT NOT NULL
);

CREATE TABLE league_rulesets (
    id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    points_per_mile INT NOT NULL,
    league_id UUID NOT NULL REFERENCES leagues (id) ON DELETE CASCADE
);

INSERT INTO leagues (id, name, start, rounds, current_round) VALUES 
  ('00000000-0000-0000-0000-000000000000', 'Test Tournament', NOW(), 10, 0);

INSERT INTO league_rulesets (league_id, points_per_mile) VALUES 
  ('00000000-0000-0000-0000-000000000000', 100);

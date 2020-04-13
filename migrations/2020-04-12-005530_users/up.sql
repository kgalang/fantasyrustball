
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id VARCHAR(36) NOT NULL PRIMARY KEY,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(122) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

insert into users (id, first_name, last_name, email, password) values 
('00000000-0000-0000-0000-000000000000', 'admin', 'user', 'admin@admin.com', '123'),
('1802d2f8-1a18-43c1-9c58-1c3f7100c842', 'test', 'user', 'test@admin.com', '123');
CREATE TABLE users (
  id UUID PRIMARY KEY,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  username VARCHAR(64) UNIQUE NOT NULL,
  password_hash VARCHAR(128) NOT NULL
);

CREATE TABLE profiles (
  id UUID UNIQUE NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  display_name VARCHAR(128) NOT NULL,
  
  avatar_id VARCHAR(32) REFERENCES files(id) ON DELETE SET NULL,
  banner_id VARCHAR(32) REFERENCES files(id) ON DELETE SET NULL,

  PRIMARY KEY(id)
);

CREATE TABLE files (
  id VARCHAR(32) PRIMARY KEY,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  mimetype VARCHAR(128) NOT NULL DEFAULT 'application/octet-stream',
  filesize BIGINT NOT NULL,

  sha256 BLOB UNIQUE NOT NULL,

  color VARCHAR(6),

  width INT,
  height INT,

  preview_allowed BOOLEAN NOT NULL DEFAULT 0,

  state VARCHAR(32) CHECK( state IN ('pending', 'processing', 'ready', 'failed') ) NOT NULL DEFAULT 'pending'
);

CREATE INDEX files_mimetype_idx ON files(mimetype);

CREATE TABLE card_files(
  card_id BIGINT NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
  file_id VARCHAR(32) NOT NULL REFERENCES files(id) ON DELETE CASCADE,

  position INT NOT NULL DEFAULT 0,

  UNIQUE(card_id, position),
  PRIMARY KEY(card_id, file_id)
);

CREATE TABLE cards (
  id BIGINT PRIMARY KEY,
  author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

  title VARCHAR(255),
  description VARCHAR(1024),

  visibility VARCHAR(32) CHECK( visibility IN ('public', 'private') ) NOT NULL DEFAULT 'public',

  deleted_at TIMESTAMPTZ
);

CREATE INDEX cards_author_id_idx ON cards(author_id);
CREATE INDEX cards_created_at_idx ON cards(created_at);
CREATE INDEX cards_visibility_idx ON cards(visibility);

CREATE TABLE cards (
  id            UUID          NOT NULL  PRIMARY KEY,

  file_id       UUID          NOT NULL  REFERENCES files(id) ON DELETE CASCADE,

  created       TIMESTAMPTZ   NOT NULL,

  title         VARCHAR(255),
  description   TEXT
);
CREATE INDEX idx_card_created ON cards(created);
CREATE INDEX idx_card_file_id ON cards(file_id);

CREATE TABLE files (
  id            UUID          NOT NULL  PRIMARY KEY,

  source_id     UUID          REFERENCES content_sources(id) ON DELETE SET NULL,
  source_url    TEXT,

  created       TIMESTAMPTZ   NOT NULL,

  sha256        BLOB          NOT NULL  UNIQUE,

  filename      VARCHAR(255),
  content_type  VARCHAR(64)   NOT NULL,
  size          BIGINT        NOT NULL
);

CREATE TABLE content_sources (
  id            UUID          NOT NULL PRIMARY KEY,
  source_domain VARCHAR(512)  NOT NULL UNIQUE
);

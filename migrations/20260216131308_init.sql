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

CREATE TABLE external_content (
  id                UUID          NOT NULL  PRIMARY KEY,
  external_id       VARCHAR(512)  NOT NULL,

  created           TIMESTAMPTZ   NOT NULL,

  title             VARCHAR(512),
  description       TEXT,

  source            VARCHAR(512)  NOT NULL,

  media_width       INTEGER,
  media_height      INTEGER,

  file_preview_url  TEXT,
  file_url          TEXT          NOT NULL,

  UNIQUE(source, external_id)
);
CREATE INDEX idx_external_content_source ON external_content(source);
CREATE INDEX idx_external_content_created ON external_content(created);

CREATE TABLE content_sources (
  id            UUID          NOT NULL PRIMARY KEY,
  source_domain VARCHAR(512)  NOT NULL UNIQUE
);

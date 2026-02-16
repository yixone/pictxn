CREATE TABLE files (
  id            UUID          NOT NULL  PRIMARY KEY,

  created       TIMESTAMPTZ   NOT NULL,

  filename      VARCHAR(255),
  content_type  VARCHAR(64)   NOT NULL  DEFAULT 'application/octet-stream',
  size          BIGINT        NOT NULL  DEFAULT 0,
  color         VARCHAR(6)
);

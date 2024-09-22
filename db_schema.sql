DROP TABLE IF EXISTS ExExBackfill;

CREATE TABLE IF NOT EXISTS ExExBackfill (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    BlockNumber INT UNIQUE,
    ArweaveHash LONGTEXT,
    BlockHash LONTEXT
);
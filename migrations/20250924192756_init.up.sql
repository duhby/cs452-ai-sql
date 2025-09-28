CREATE TABLE member
(
    id       TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL
);

CREATE TYPE file_type AS ENUM (
    'image/png',
    'image/jpeg',
    'audio/mpeg',
    'text/plain',
    'video/mp4'
);

-- id and type_ are used to create a cdn url, example: cdn.example.com/abc.mp4
CREATE TABLE file
(
    id     TEXT PRIMARY KEY,
    member TEXT NOT NULL REFERENCES member(id),
    -- non-unique original name of the file
    name   TEXT NOT NULL,
    type_  file_type NOT NULL
);

CREATE TABLE tag
(
    name  TEXT PRIMARY KEY,
    -- Hex code
    color VARCHAR(6) NOT NULL
);

CREATE TABLE tag_parent (
    child TEXT REFERENCES tag(name),
    parent TEXT REFERENCES tag(name),
    PRIMARY KEY (child, parent)
);

CREATE TABLE file_tag (
    file TEXT REFERENCES file(id),
    tag TEXT REFERENCES tag(name),
    PRIMARY KEY (file, tag)
);

INSERT INTO member (id, username) VALUES
       ('m1', 'alice'),
       ('m2', 'bob'),
       ('m3', 'carol'),
       ('m4', 'dave'),
       ('m5', 'eve');

INSERT INTO file (id, member, name, type_) VALUES
       ('f1', 'm1', 'funny cat', 'image/png'),
       ('f2', 'm2', 'insane clip', 'video/mp4'),
       ('f3', 'm3', 'normal cat', 'audio/mpeg'),
       ('f4', 'm4', 'minecraft plan', 'text/plain'),
       ('f5', 'm5', 'random meme', 'image/jpeg');

INSERT INTO tag (name, color) VALUES
       ('meme', 'FFAA00'),
       ('clip', '00AAFF'),
       ('cat', 'FF00AA'),
       ('video_game', '00FFAA');

INSERT INTO tag_parent (child, parent) VALUES
       ('clip', 'video_game');

INSERT INTO file_tag (file, tag) VALUES
       ('f1', 'meme'),
       ('f1', 'cat'),
       ('f2', 'clip'),
       ('f3', 'cat'),
       ('f4', 'video_game'),
       ('f5', 'meme');

DELETE FROM file_tag WHERE file IN ('f1','f2','f3','f4','f5');
DELETE FROM tag_parent WHERE child = 'clip' AND parent = 'video_game';
DELETE FROM file WHERE id IN ('f1','f2','f3','f4','f5');
DELETE FROM tag WHERE name IN ('meme','clip','cat','video_game');
DELETE FROM member WHERE id IN ('m1','m2','m3','m4','m5');

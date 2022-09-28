-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_(
    id INT PRIMARY KEY,
    name VARCHAR(20) NOT NULL
);

CREATE TABLE IF NOT EXISTS bit_image(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description VARCHAR(300) NOT NULL,
    create_at TIMESTAMP NOT NULL DEFAULT now(),
    update_at TIMESTAMP NOT NULL DEFAULT now(),
    user_id INT REFERENCES user_ NOT NULL
);
CREATE TABLE IF NOT EXISTS bit_image_like (
    bit_image_id int REFERENCES bit_image ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    user_id int REFERENCES user_ ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    score SMALLINT NOT NULL,
    create_at TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id,bit_image_id)
);

CREATE INDEX ON bit_image_like(bit_image_id);
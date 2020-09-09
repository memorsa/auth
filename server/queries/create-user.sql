INSERT INTO users (name, password_digest)
VALUES ( $1, $2 )
RETURNING id

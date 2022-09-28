//All Columns
//id, username, role, create_time, update_time

pub const GET_BY_USERNAME: &str = "
    SELECT
        id, username, role, create_time, update_time 
    FROM users
    WHERE
        username = $1
";

pub const INSERT: &str = "
    INSERT INTO users
        (username, role, create_time, update_time)
    VALUES
        ($1, $2, $3, $4)
    RETURNING
        id, username, role, create_time, update_time
";

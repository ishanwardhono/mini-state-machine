//All Columns
//id, username, role, create_time, create_by, update_time, update_by

pub const GET_BY_USERNAME: &str = "
    SELECT
        id, username, role, create_time, create_by, update_time, update_by 
    FROM users
    WHERE
        username = $1
";

pub const INSERT: &str = "
    INSERT INTO users
        (username, role, create_time, create_by, update_time, update_by)
    VALUES
        ($1, $2, $3, $4, $5, $6)
    RETURNING
        id, username, role, create_time, create_by, update_time, update_by
";

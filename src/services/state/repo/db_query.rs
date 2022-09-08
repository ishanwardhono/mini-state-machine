pub const SELECT_ALL: &str = "
    SELECT * FROM states
";

pub const SELECT_BY_ID: &str = "
    SELECT * FROM states WHERE id = $1
";

pub const INSERT: &str = "
    INSERT INTO states 
        (code, description, webhooks, update_time, create_time)
    VALUES
        ($1,$2,$3,$4,$5)
";

pub const UPDATE: &str = "
    UPDATE states SET
        (code, description, webhooks, update_time) = ($2,$3,$4,$5)
    WHERE
        id = $1
";

pub const DELETE: &str = "
    DELETE FROM states WHERE id = $1
";

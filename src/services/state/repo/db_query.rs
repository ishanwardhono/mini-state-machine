pub const SELECT_ALL: &str = "
    SELECT * FROM states
";

pub const SELECT_BY_CODE: &str = "
    SELECT * FROM states WHERE code = $1
";

pub const INSERT: &str = "
    INSERT INTO states 
        (code, description, webhooks, update_time, create_time)
    VALUES
        ($1,$2,$3,$4,$5)
";

pub const UPDATE: &str = "
    UPDATE states SET
        (description, webhooks, update_time) = ($2,$3,$4)
    WHERE
        code = $1
";

pub const DELETE: &str = "
    DELETE FROM states WHERE code = $1
";

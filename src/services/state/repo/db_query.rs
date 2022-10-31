//All Columns
//id, code, description, actions, create_time, create_by, update_time, update_by

pub const SELECT_ALL: &str = "
    SELECT 
        id, code, description, actions, create_time, create_by, update_time, update_by 
    FROM states
";

pub const SELECT_BY_CODE: &str = "
    SELECT 
        id, code, description, actions, create_time, create_by, update_time, update_by 
    FROM states
    WHERE code = $1
";

pub const SELECT_BY_CODES: &str = "
    SELECT code
    FROM states
    WHERE code = ANY($1)
";

pub const INSERT: &str = "
    INSERT INTO states 
        (id, code, description, actions, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
    RETURNING
        id, code, description, actions, create_time, create_by, update_time, update_by 
";

pub const UPDATE: &str = "
    UPDATE states SET
        (description, actions, update_time, update_by) = ($2,$3,$4,$5)
    WHERE
        code = $1
    RETURNING
        id, code, description, actions, create_time, create_by, update_time, update_by 
";

pub const DELETE: &str = "
    DELETE FROM states WHERE code = $1
";

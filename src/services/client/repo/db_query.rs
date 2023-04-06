//All Columns
//id, code, url, auth_token, create_time, create_by, update_time, update_by

pub const SELECT_BY_CODE: &str = "
    SELECT 
        id, code, url, auth_token, create_time, create_by, update_time, update_by 
    FROM clients
    WHERE code = $1
";

pub const SELECT_BY_CODES: &str = "
    SELECT code
    FROM clients
    WHERE code = ANY($1)
";

pub const INSERT: &str = "
    INSERT INTO clients 
        (id, code, url, auth_token, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
    RETURNING
        code
";

pub const UPDATE: &str = "
    UPDATE clients SET
        (url, auth_token, update_time, update_by) = ($2,$3,$4,$5)
    WHERE
        code = $1
    RETURNING
        code
";

pub const DELETE: &str = "
    DELETE FROM clients WHERE code = $1
";

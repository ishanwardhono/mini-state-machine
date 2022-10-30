//All Columns
//id, code, url, create_time, create_by, update_time, update_by

pub const SELECT_BY_CODE: &str = "
    SELECT 
        id, code, url, create_time, create_by, update_time, update_by 
    FROM clients
    WHERE code = $1
";

pub const INSERT: &str = "
    INSERT INTO clients 
        (id, code, url, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7)
    RETURNING
        code
";

pub const UPDATE: &str = "
    UPDATE clients SET
        (url, update_time, update_by) = ($2,$3,$4)
    WHERE
        code = $1
    RETURNING
        code
";

pub const DELETE: &str = "
    DELETE FROM clients WHERE code = $1
";

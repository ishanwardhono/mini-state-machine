//All Columns
//id, business, order_id, from_state, to_state, create_time, create_by, update_time, update_by

pub const INSERT: &str = "
    INSERT INTO retry_actions
        (id, client, business, order_id, from_state, to_state, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
";

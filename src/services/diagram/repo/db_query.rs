pub const BUSINESS_INSERT: &str = "
    INSERT INTO business
        (id, code, description, is_active, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
";

pub const FLOW_INSERT: &str = "
    INSERT INTO flows 
        (id, business, state, is_initial_state, next_states, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8,$9)
";

pub const ORDER_INSERT: &str = "
    INSERT INTO orders
        (id, client_order_id, business, state, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
";

pub const ORDER_STATE_UPDATE: &str = "
    UPDATE orders
    SET 
        (state, update_time, update_by)
        = ($2,$3,$4)
    WHERE id = $1
";

pub const ORDER_GET: &str = "
    SELECT
        id, client_order_id, business, state, create_time, create_by, update_time, update_by
    FROM orders
    WHERE
        id = $1
";

pub const ORDER_GET_BY_CLIENT_ORDER_ID: &str = "
    SELECT
        id, client_order_id, business, state, create_time, create_by, update_time, update_by
    FROM orders
    WHERE
        business = $1 AND client_order_id = $2
";

pub const ORDER_EXISTS_BY_CLIENT_ORDER_ID: &str = "
    SELECT EXISTS (
        SELECT
            id, client_order_id, business, state, create_time, create_by, update_time, update_by
        FROM orders
        WHERE
            business = $1 AND client_order_id = $2
    )
";

pub const HISTORY_INSERT: &str = "
    INSERT INTO histories
        (id, order_id, from_state, to_state, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
";

pub const HISTORY_GET: &str = "
    SELECT
        from_state, to_state, create_time, create_by
    FROM histories
    WHERE
        order_id = $1
    ORDER BY create_time ASC
";

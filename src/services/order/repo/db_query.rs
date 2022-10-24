pub const ORDER_INSERT: &str = "
    INSERT INTO orders
        (id, order_id, business, state, create_time, create_by, update_time, update_by)
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
        id, order_id, business, state, create_time, create_by, update_time, update_by
    FROM orders
    WHERE
        id = $1
";

pub const ORDER_GET_BY_ORDER_ID: &str = "
    SELECT
        id, order_id, business, state, create_time, create_by, update_time, update_by
    FROM orders
    WHERE
        business = $1 AND order_id = $2
";

pub const ORDER_EXISTS_BY_ORDER_ID: &str = "
    SELECT EXISTS (
        SELECT
            id, order_id, business, state, create_time, create_by, update_time, update_by
        FROM orders
        WHERE
            business = $1 AND order_id = $2
    )
";

pub const ORDER_INSERT: &str = "
    INSERT INTO orders
        (id, order_id, business, state, create_time, create_by, update_time, update_by)
    VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8)
";

pub const ORDER_GET: &str = "
    SELECT
        id, order_id, business, state, create_time, create_by, update_time, update_by
    FROM orders
    WHERE
        id = $1
";

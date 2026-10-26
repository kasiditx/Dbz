SELECT o.id, o.status, o.total, c.name AS ลูกค้า
FROM app.orders o
JOIN app.customers c ON c.id = o.customer_id
WHERE o.id > :min_id -- synthetic only; no database connection
ORDER BY o.id;


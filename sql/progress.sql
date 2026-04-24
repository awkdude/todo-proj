SELECT COALESCE(SUM(t.id), 0)
FROM task AS t
JOIN recurring_task AS p ON t.prototype_id = p.id
WHERE (t.user_id = {user_id})

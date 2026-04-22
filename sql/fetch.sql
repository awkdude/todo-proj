SELECT t.id, t.prototype_id, t.user_id, t.completion_value, t.due_date, p.id, p.title, p.frequency_type, p.frequency_value, p.is_range, p.completion_max
FROM task AS t 
JOIN recurring_task AS p ON t.prototype_id = p.id
WHERE p.user_id = {user_id}

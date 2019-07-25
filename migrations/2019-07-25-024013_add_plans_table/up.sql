CREATE TABLE plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    description VARCHAR,
    completed BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY(task_id) REFERENCES tasks(id)
);


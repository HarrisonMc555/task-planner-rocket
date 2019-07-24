CREATE TABLE plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    taskid INTEGER NOT NULL,
    FOREIGN KEY(taskid) REFERENCES tasks(id),
    description VARCHAR,
    completed BOOLEAN NOT NULL DEFAULT 0
);


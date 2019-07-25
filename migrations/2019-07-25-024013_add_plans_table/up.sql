CREATE TABLE plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    taskid INTEGER NOT NULL,
    description VARCHAR,
    completed BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY(taskid) REFERENCES tasks(id)
);


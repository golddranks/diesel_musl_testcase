CREATE TABLE members (
    id VARCHAR PRIMARY KEY,
    employee_num INTEGER,
    name VARCHAR NOT NULL,
    leader BOOLEAN NOT NULL,
    mail VARCHAR NOT NULL,
    phone VARCHAR NOT NULL,
    business_connection VARCHAR NOT NULL,
    memo VARCHAR NOT NULL
);

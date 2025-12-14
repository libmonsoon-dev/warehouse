CREATE TYPE resource_action AS ENUM ('create', 'read', 'list', 'update', 'delete');

CREATE TYPE resource_type AS ENUM ('user', 'role', 'user_role', 'rule', 'role_rule');

CREATE TYPE rule_effect AS ENUM ('allow', 'deny');

CREATE TABLE "roles"
(
    "id"          UUID                NOT NULL PRIMARY KEY,
    "name"        VARCHAR(100) UNIQUE NOT NULL,
    "description" TEXT
);

CREATE TABLE "user_roles"
(
    "user_id"     UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "role_id"     UUID NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    "assigned_by" UUID REFERENCES users (id),
    PRIMARY KEY ("user_id", "role_id")
);

CREATE TABLE "rules"
(
    "id"            UUID            NOT NULL PRIMARY KEY,
    "action"        resource_action NOT NULL,
    "resource_type" resource_type   NOT NULL,
    "effect"        rule_effect     NOT NULL
);

CREATE TABLE "role_rules"
(
    "role_id"     UUID NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    "rule_id"     UUID NOT NULL REFERENCES rules (id) ON DELETE CASCADE,
    "assigned_by" UUID,
    PRIMARY KEY ("role_id", "rule_id")
);

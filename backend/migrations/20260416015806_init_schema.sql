-- Add migration script here
BEGIN;

CREATE TABLE dorms (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    code TEXT UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    display_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    deactivated_at TIMESTAMPTZ
);

CREATE TABLE user_identities (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_subject TEXT NOT NULL,
    email TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT user_identities_provider_not_blank
        CHECK (btrim(provider) <> ''),
    CONSTRAINT user_identities_subject_not_blank
        CHECK (btrim(provider_subject) <> ''),

    CONSTRAINT user_identities_provider_subject_unique
        UNIQUE (provider, provider_subject),

    CONSTRAINT user_identities_one_identity_per_provider_per_user
        UNIQUE (user_id, provider)
);

CREATE TABLE user_dorm_memberships (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    dorm_id BIGINT NOT NULL REFERENCES dorms(id) ON DELETE RESTRICT,
    start_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    end_at TIMESTAMPTZ,

    CONSTRAINT user_dorm_memberships_valid_range
        CHECK (end_at IS NULL OR end_at > start_at)
);

CREATE INDEX user_dorm_memberships_user_id_idx
    ON user_dorm_memberships (user_id);

CREATE INDEX user_dorm_memberships_dorm_id_idx
    ON user_dorm_memberships (dorm_id);

CREATE UNIQUE INDEX user_dorm_memberships_one_active_dorm_per_user_idx
    ON user_dorm_memberships (user_id)
    WHERE end_at IS NULL;

COMMIT;
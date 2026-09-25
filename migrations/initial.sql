PRAGMA foreign_keys = ON;

CREATE TABLE Run (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL, -- Human-readable name of the run. It should be unique to ensure a good user experience, but this is not enforced.
    reference TEXT DEFAULT NULL -- Absolute ISO8601 reference timestamp. All sample times are measured in nanoseconds since this timestamp (possible negative).
) STRICT;

CREATE TABLE Attribute (
    id INTEGER PRIMARY KEY,
    key TEXT NOT NULL, -- Technical attribute identifier. If attributes are displayed to the user, this needs to be translated to a human readable string by the application.
    value TEXT DEFAULT NULL, -- Machine readable value of the attribute. The format depends on the attribute.
    run_id INTEGER REFERENCES Run (id) ON DELETE CASCADE, -- Every [Run] can has its own set of [Attribute]s.

    UNIQUE (run_id, key) -- An attribute key must be unique per run. So every run can have exactly zero or one entries per key.
) STRICT;

CREATE TABLE Signal (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL, -- Technical name of the signal. The name should not contain the run's name in it, so two signals from two runs can have the same name and therefore treated like the same signal by the UI.
    description TEXT DEFAULT NULL, -- Human readable description of a signal.
    unit TEXT DEFAULT NULL, -- Unit symbol if any. Should follow the [IEEE recommendation](https://de.scribd.com/document/543894605/Abbreviation).
    run_id INTEGER NOT NULL REFERENCES Run (id) ON DELETE CASCADE
) STRICT;

CREATE TABLE Timestamps (
    id INTEGER PRIMARY KEY,
    reference TEXT DEFAULT NULL, -- Absolute ISO8601 timestamp of the first sample.
    begin INTEGER NOT NULL, -- Timestamp of the first sample of this track, measured in nanoseconds since the run's reference timestamp.
    end INTEGER NOT NULL, -- Timestamp of the last sample of this track, measured in nanoseconds since the run's reference timestamp.
    data BLOB DEFAULT NULL, -- Potentially compressed sample times.
    compression TEXT NOT NULL DEFAULT 'none', -- Data compression applied to `data`.
    run_id INTEGER NOT NULL REFERENCES Run (id) ON DELETE CASCADE
) STRICT;

CREATE TABLE Track (
    id INTEGER PRIMARY KEY,
    signal_id INTEGER NOT NULL REFERENCES Signal (id) ON DELETE CASCADE,
    timestamps_id INTEGER NOT NULL REFERENCES Timestamps (id) ON DELETE CASCADE,
    data BLOB DEFAULT NULL, -- Potentially compressed sample values.
    minimum REAL DEFAULT NULL, -- Lowest observed value in this track if any.
    maximum REAL DEFAULT NULL, -- Highest observed value in this track if any.
    average REAL DEFAULT NULL, -- Average observed value in this track if any.
    count INTEGER NOT NULL DEFAULT 0 -- Number of samples in this track. Zero if no samples has been recorded.
) STRICT;

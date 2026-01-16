CREATE TABLE reservations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    room_id INTEGER NOT NULL REFERENCES rooms(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CHECK (start_time < end_time)
);

CREATE INDEX idx_reservations_room_time ON reservations(room_id, start_time, end_time);
CREATE INDEX idx_reservations_user ON reservations(user_id);

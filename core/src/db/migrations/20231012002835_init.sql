-- Create msg table

CREATE TABLE IF NOT EXISTS msg (
  id BLOB PRIMARY KEY CHECK(length(id) = 16) NOT NULL,
  channel_id BLOB CHECK(length(id) = 16),
  role TEXT NOT NULL,
  cur_idx INTEGER NOT NULL,
  cont_list TEXT NOT NULL,
  meta TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
  updated_at TIMESTAMP NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
  UNIQUE(id)
);

-- Create channel table

CREATE TABLE IF NOT EXISTS channel (
  id BLOB PRIMARY KEY CHECK(length(id) = 16) NOT NULL,
  name TEXT NOT NULL,
  desc TEXT,
  cfg TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
  updated_at TIMESTAMP NOT NULL DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')),
  UNIQUE(id)
);

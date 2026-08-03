CREATE TABLE IF NOT EXISTS page_views (
    slug TEXT PRIMARY KEY,
    view_count INTEGER NOT NULL DEFAULT 1,
    last_viewed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_page_views_count ON page_views (view_count DESC);

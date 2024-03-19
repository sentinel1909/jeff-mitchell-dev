-- migration to create the articles table

CREATE TABLE IF NOT EXISTS articles (
  article_id SERIAL PRIMARY KEY,
  article_title TEXT UNIQUE,
  article_date TEXT,
  article_slug TEXT,
  article_category TEXT,
  article_tag TEXT,
  article_summary TEXT,
  article_content TEXT
);
-- migration to create the articles table

CREATE TABLE IF NOT EXISTS articles (
  article_id SERIAL PRIMARY KEY,
  article_title TEXT NOT NULL UNIQUE,
  article_date TEXT NOT NULL,
  article_slug TEXT NOT NULL,
  article_category TEXT NOT NULL,
  article_tag TEXT NOT NULL,
  article_summary TEXT NOT NULL,
  article_content TEXT NOT NULL
);
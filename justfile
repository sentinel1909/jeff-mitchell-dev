# jeff-mitchell-dev just command runner configuration

build:
  cd site && ./zola build --output-dir ../public --force

dev:
  cd site && ./zola serve --drafts


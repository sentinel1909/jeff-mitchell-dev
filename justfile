# jeff-mitchell-dev just command runner configuration

build-site:
  cd site && ./zola build --output-dir ../public --force

serve-dev:
  cd site && zola serve


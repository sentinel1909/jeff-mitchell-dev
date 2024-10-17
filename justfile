# jeff-mitchell-dev just command runner configuration

# use PowerShell instead of sh
set shell := ["powershell.exe", "-c"]

build:
  cd site; ./zola build --output-dir ../public --force

dev:
  cd site;  ./zola serve --drafts


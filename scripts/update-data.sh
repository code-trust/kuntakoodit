#!/usr/bin/env bash
# Downloads the currently effective Statistics Finland municipality
# classification (kunta_1_YYYYMMDD) into data/. Output is pretty-printed so
# diffs stay readable.
set -euo pipefail

API="https://data.stat.fi/api/classifications/v2/classifications"
DATA_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/data"
TODAY="$(date -u +%Y-%m-%d)"
TODAY_COMPACT="$(date -u +%Y%m%d)"

classification="$(
  curl -fsS "${API}?content=data&meta=max&lang=fi&format=json" |
    jq -r --arg today "$TODAY" --arg today_compact "$TODAY_COMPACT" '
      # The ID encodes the date the classification takes effect, which can be
      # later than its release.
      [.[] | select(
        (.localId | startswith("kunta_1_"))
        and .releaseDate <= $today
        and .localId[8:] <= $today_compact
      )]
      | max_by(.localId)
      | .localId // empty'
)"

if [[ -z "$classification" ]]; then
  echo "no effective kunta_1 classification found" >&2
  exit 1
fi

echo "latest classification: ${classification}"
mkdir -p "$DATA_DIR"

for lang in fi sv; do
  tmp="$(mktemp)"
  curl -fsS "${API}/${classification}/classificationItems?content=data&meta=max&lang=${lang}&format=json" |
    jq '.' >"$tmp"

  count="$(jq 'length' "$tmp")"
  if [[ "$count" -eq 0 ]]; then
    echo "empty response for lang=${lang}" >&2
    rm -f "$tmp"
    exit 1
  fi

  chmod 644 "$tmp"
  mv "$tmp" "${DATA_DIR}/kunnat_${lang}.json"
  echo "wrote ${count} municipalities to data/kunnat_${lang}.json"
done

name="$(jq -r '.[0].classification.classificationName[0].name' "${DATA_DIR}/kunnat_fi.json")"
cat >"${DATA_DIR}/ATTRIBUTION.md" <<EOF
# Data attribution

Source: Statistics Finland, ${name} (\`${classification}\`).

- Classification: https://stat.fi/en/luokitukset/kunta/${classification}
- License: [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)

\`kunnat_fi.json\` and \`kunnat_sv.json\` are Statistics Finland classification API
responses, reformatted with \`jq\`.
EOF
echo "wrote data/ATTRIBUTION.md"

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "classification=${classification}" >>"$GITHUB_OUTPUT"
fi

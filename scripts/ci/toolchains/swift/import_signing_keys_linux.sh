set -euo pipefail
curl --fail --location \
  --retry 3 --retry-all-errors --retry-delay 1 --retry-max-time 180 \
  --connect-timeout 15 --max-time 60 --silent --show-error --compressed \
  https://www.swift.org/keys/all-keys.asc | gpg --batch --import -
for expected in \
  E813C892820A6FA13755B268F167DF1ACF9CE069 \
  52BB7E3DE28A71BE22EC05FFEF80A866B47A981F; do
  actual="$(gpg --batch --with-colons --fingerprint "$expected" | awk -F: '$1 == "fpr" { print $10; exit }')"
  if [[ "$actual" != "$expected" ]]; then
    echo "Expected official Swift signing key $expected was not imported" >&2
    exit 1
  fi
done


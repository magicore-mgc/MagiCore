set -euo pipefail
brew update
brew install php
php_bin="$(brew --prefix php)/bin"
echo "${php_bin}" >> "${GITHUB_PATH}"
"${php_bin}/php" --version


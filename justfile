set positional-arguments

watch +args='test':
  cargo watch --clear --exec '{{args}}'

ci: clippy forbid
  cargo fmt -- --check
  cargo test --all
  cargo test --all -- --ignored

forbid:
  ./bin/forbid

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all --all-targets -- --deny warnings

deploy branch remote chain domain:
  ssh root@{{domain}} 'mkdir -p deploy \
    && apt-get update --yes \
    && apt-get upgrade --yes \
    && apt-get install --yes git rsync'
  rsync -avz deploy/checkout root@{{domain}}:deploy/checkout
  ssh root@{{domain}} 'cd deploy && ./checkout {{branch}} {{remote}} {{chain}} {{domain}}'

deploy-mainnet branch='master' remote='Groestlcoin/ord-groestlcoin': (deploy branch remote 'main' 'ordinals.groestlcoin.org')

deploy-signet branch='master' remote='Groestlcoin/ord-groestlcoin': (deploy branch remote 'signet' 'ordinals-signet.groestlcoin.org')

deploy-testnet branch='master' remote='Groestlcoin/ord-groestlcoin': (deploy branch remote 'test' 'ordinals-test.groestlcoin.org')

deploy-all: \
  deploy-regtest \
  deploy-testnet \
  deploy-signet \
  deploy-mainnet-alpha \
  deploy-mainnet-bravo \
  deploy-mainnet-charlie

servers := 'alpha bravo charlie regtest signet testnet'

initialize-server-keys:
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/ssh
  mkdir -p tmp/ssh
  ssh-keygen -C ordinals -f tmp/ssh/id_ed25519 -t ed25519 -N ''
  for server in {{ servers }}; do
    ssh-copy-id -i tmp/ssh/id_ed25519.pub root@$server.groestlcoin.org
    scp tmp/ssh/* root@$server.groestlcoin.org:.ssh
  done
  rm -rf tmp/ssh

install-personal-key key='~/.ssh/id_ed25519.pub':
  #!/usr/bin/env bash
  set -euxo pipefail
  for server in {{ servers }}; do
    ssh-copy-id -i {{ key }} root@$server.groestlcoin.org
  done

server-keys:
  #!/usr/bin/env bash
  set -euxo pipefail
  for server in {{ servers }}; do
    ssh root@$server.groestlcoin.org cat .ssh/authorized_keys
  done

log unit='ord' domain='alpha.groestlcoin.org':
  ssh root@{{domain}} 'journalctl -fu {{unit}}'

fuzz:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd fuzz
  while true; do
    cargo +nightly fuzz run transaction-builder -- -max_total_time=60
    cargo +nightly fuzz run runestone-decipher -- -max_total_time=60
    cargo +nightly fuzz run varint-decode -- -max_total_time=60
    cargo +nightly fuzz run varint-encode -- -max_total_time=60
  done

open:
  open http://localhost

doc:
  cargo doc --all --open

prepare-release revision='master':
  #!/usr/bin/env bash
  set -euxo pipefail
  git checkout {{ revision }}
  git pull origin {{ revision }}
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md
  $EDITOR CHANGELOG.md
  $EDITOR Cargo.toml
  version=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  cargo check
  git checkout -b release-$version
  git add -u
  git commit -m "Release $version"
  gh pr create --web

publish-release revision='master':
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone https://github.com/Groestlcoin/ord-groestlcoin.git tmp/release
  cd tmp/release
  git checkout {{ revision }}
  cargo publish
  cd ../..
  rm -rf tmp/release

publish-tag-and-crate revision='master':
  #!/usr/bin/env bash
  set -euxo pipefail
  rm -rf tmp/release
  git clone git@github.com:Groestlcoin/ord-groestlcoin.git tmp/release
  cd tmp/release
  git checkout {{revision}}
  version=`sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
  git tag -a $version -m "Release $version"
  git push git@github.com:Groestlcoin/ord-groestlcoin.git $version
  cargo publish
  cd ../..
  rm -rf tmp/release

list-outdated-dependencies:
  cargo outdated -R
  cd test-groestlcoincore-rpc && cargo outdated -R

update-modern-normalize:
  curl \
    https://raw.githubusercontent.com/sindresorhus/modern-normalize/main/modern-normalize.css \
    > static/modern-normalize.css

download-log unit='ord' host='alpha.groestlcoin.org':
  ssh root@{{host}} 'mkdir -p tmp && journalctl -u {{unit}} > tmp/{{unit}}.log'
  rsync --progress --compress root@{{host}}:tmp/{{unit}}.log tmp/{{unit}}.log

graph log:
  ./bin/graph $1

flamegraph dir=`git branch --show-current`:
  ./bin/flamegraph $1

serve-docs: build-docs
  open http://127.0.0.1:8080
  python3 -m http.server --directory docs/build/html --bind 127.0.0.1 8080

build-docs:
  #!/usr/bin/env bash
  mdbook build docs -d build
  for language in ar de es fil fr hi it ja ko pt ru zh; do
    MDBOOK_BOOK__LANGUAGE=$language mdbook build docs -d build/$language
    mv docs/build/$language/html docs/build/html/$language
  done

update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

preview-examples:
  cargo run preview examples/*

convert-logo-to-favicon:
  convert -background none -resize 256x256 logo.svg static/favicon.png

update-mdbook-theme:
  curl https://raw.githubusercontent.com/rust-lang/mdBook/v0.4.35/src/theme/index.hbs > docs/theme/index.hbs

audit-cache:
  cargo run --package audit-cache

coverage:
  cargo llvm-cov

benchmark-server:
  cargo bench --bench server

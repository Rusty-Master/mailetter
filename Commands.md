# SQLX offline mode

## create sqlx-data.json for all queries (tests included)
cargo sqlx prepare -- --all-targets --all-features


## create sqlx-data.json for lib
cargo sqlx prepare -- --lib

- connection to db necessary.
- to enforce using offline mode set env variable: SQLX_OFFLINE = true 

## check if up to date
cargo sqlx prepare --check

## sqlx-data.json no queries
cargo install sqlx-cli && cargo sqlx prepare -- --all-targets --all-features


# If docker already running:
SKIP_DOCKER=true ./scripts/init_db.sh


# To see logs in tests / using bunyan to prettify logs
TEST_LOG=true cargo test | bunyan

# Illegal option -o pipefail
bash init_db.sh
or
chmod +x init_db.sh
./init_db.sh

# If error 'Cannot connect to the Docker daemon at unix:///var/run/docker.sock. Is the docker daemon running?.'
Open Docker desktop go to:
Settings -> Resources -> WSL Intergration

Check ubuntu swithch

It resets often, do not know why
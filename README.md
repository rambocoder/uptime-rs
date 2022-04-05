
# Uptime - monitor your apps

# Build and run
```
cargo install cargo-watch
cargo watch -x run
```

# To see documentation of all deps
```
cargo docserver
```

CARGO_TARGET_DIR=central loc where all the crates compiles will store data

# Create shared cargo target tmpfs
```
mkdir ~/target
cd ~
sudo mount -t tmpfs none ~/target
# create shared tmpfs
cat /proc/mounts | rg "$(pwd)" | sudo tee -a /etc/fstab
export CARGO_TARGET_DIR="$HOME/target"
# or instead of shared one
ln ~/target target -s
```
# install lld linker
```
sudo apt-get install -y lld clang mold
```

# Run PostgreSQL in container
```
mkdir -p ~/pgdata
```
Then start PostgreSQL using a volume mount so the container will store the data in this newly created local directory:
```
docker run \
  -d \
  --name postgresql-container \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -v ~/pgdata:/var/lib/postgresql/data \
  postgres
```
Connect to container
```
docker exec -it postgresql-container bash
```
Connect to PgSQL
```
psql -h localhost -p 5432 -U postgres -W
```
Create database
```
CREATE DATABASE uptime;
```

https://www.youtube.com/watch?v=yNe9Xr35n4Q
https://collectednotes.com/javier/let-s-create-a-basic-crud-with-rust-using-tide

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
sudo mount -t tmpfs none ~/target && 
# create shared tmpfs
cat /proc/mounts | rg "$(pwd)" | sudo tee -a /etc/fstab
export CARGO_TARGET_DIR="$HOME/target"
# or instead of shared one
ln ~/target target -s
```
# install lld linker
```
sudo apt-get install -y lld
```
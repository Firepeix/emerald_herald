
help:
    @just --list
run:
    @export $(grep -v '^#' .env | sed -e 's/ //g' -e "s/'//g") && cargo build; cargo run
serve:
    -@emerald_herald >> storage/log/emerald_herald.log
release:
    cargo build --release  
    cp target/release/emerald_herald bin/emerald_herald
apply:
    docker-compose up -d --build    
test-env:
    @export $(grep -v '^#' .env | sed -e 's/ //g' -e "s/'//g") && export
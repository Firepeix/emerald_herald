
run:
    @cargo build; cargo run
serve:
    -@emerald_herald >> storage/log/emerald_herald.log
release:
    @cargo build --release    
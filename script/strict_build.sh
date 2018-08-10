REPOROOT="$(git rev-parse --show-toplevel)"
cd $REPOROOT

listen() {
    RESULT="$( $@ )"
    [ $? -ne 0 ] && echo "\e[91mSTRICT BUILD FAILED '$@' \e[0m" && exit 1
}

listen rustdoc --test README.md
listen cargo test
listen cargo +nightly clippy
listen cargo build --release
listen cargo tarpaulin -v

listen sh script/build_example_output.sh

echo "\e[92mSTRICT BUILD PASSED \e[0m"


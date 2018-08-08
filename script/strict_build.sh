REPOROOT="$(git rev-parse --show-toplevel)"
cd $REPOROOT

listen() {
    RESULT="$( $@ )"
    if [ $? -ne 0 ]; then
        echo "\e[91mSTRICT BUILD FAILED '$@' \e[0m"
        exit 1
    fi
}

#listen rustdoc --test README.md
#listen cargo test
#listen cargo +nightly clippy
#listen cargo build --release
#listen sh script/build_example_output.sh

cargo tarpaulin -v


echo "\e[92mSTRICT BUILD PASSED \e[0m"

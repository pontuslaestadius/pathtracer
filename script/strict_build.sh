REPOROOT="$(git rev-parse --show-toplevel)"
cd $REPOROOT

listen() {
    cd $REPOROOT
    EXIT=$1
    shift
    start=`date +%s`
    RESULT="$( $@ )"

    if [ $? -ne $EXIT ]; then
        STAT="\e[91mFAILED"
    else
        STAT="\e[92mok"
    fi
    end=`date +%s`
    runtime=$((end-start))
    echo "$STAT\e[0m\t\t$runtime\t\t$@" >> out.build
}

echo "RESULT\t\tTIME\t\tCOMMAND" >> out.build

listen 1 grep "\s$" -r -nr --include \*.rs
listen 0 cargo build --release
listen 0 cargo test
listen 0 cargo +nightly clippy
listen 0 cargo tarpaulin -v
listen 0 sh script/build_example_output.sh

cat out.build
rm out.build

REPOROOT="$(git rev-parse --show-toplevel)"
cd "$REPOROOT/examples"
rm out/*

for file in *.rs
do
    file="${file%.*}"
    cargo run --example $file & 2> /dev/null
    pid=$!

    while kill -0 $pid 2> /dev/null; do sleep 0.5; done;

    sleep 1
    for out in $PWD/out.*
    do
        ext="${out##*.}"
        mv "$out" "out/$file.$ext"
    done
done


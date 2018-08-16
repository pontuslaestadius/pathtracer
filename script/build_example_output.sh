cd examples
rm out/*

for file in *.rs
do
    file="${file%.*}"
    cargo run --example $file
    [ $? -ne 0 ] && exit 1

    for out in $PWD/out.*
    do
        ext="${out##*.}"
        mv "$out" "out/$file.$ext"
    done
done


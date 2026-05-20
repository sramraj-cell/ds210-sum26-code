#1/bin/bash
if [ ! -d "example" ]; then
    echo "creating example project"
    cargo new example
fi

cd example
cargo run

echo "Feel free to open example in your VSCode!"

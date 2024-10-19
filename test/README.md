curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

. "$HOME/.cargo/env"

cd ..

make compiler-release

cd ..

git clone git://c9x.me/qbe.git

cd qbe

make

export PATH="$(pwd):$PATH"

cd ../elle

./ellec examples/fun/card.l --emit-qbe

./ellec examples/fun/card.l --emit-asm

./ellec examples/hello.l --emit-asm

gcc -c hello.s -o hello.o

gcc hello.o -o hello

./hello
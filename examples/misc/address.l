use std/io;

fn other(i32 *a, string *str) {
    io::printf("(fn other)\n\ta = {}\n\tstr = {}", *a, *str);
    *a = 542;
}

fn main() {
    i32 a = 39;
    string str = "Hello world!";

    other(&a, &str);
    io::printf("(fn main)\n\ta = {}", a);
}

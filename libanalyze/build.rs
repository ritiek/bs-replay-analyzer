fn main() {
    cc::Build::new()
        .cpp(true)
        .file("libbrp/src/huffman.cpp")
        .file("libbrp/src/libbrp.cpp")
        .compile("libbrp.a");
}

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("libdecompress/src/huffman.cpp")
        .file("libdecompress/src/decompress.cpp")
        .compile("libdecompress.a");
}

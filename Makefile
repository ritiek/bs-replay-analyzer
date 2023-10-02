GNU ?= g++
ARGS ?= -Wall
OPTIMIZATIONS ?= -O2

SRC = ./src
TESTS =
BIN = ./bin
LIB = ./lib

all : lib bin

lib :
	mkdir -p $(LIB)
	$(GNU) $(ARGS) $(OPTIMIZATIONS) -c $(SRC)/huffman.cpp -o $(LIB)/huffman.o
	$(GNU) $(ARGS) $(OPTIMIZATIONS) -c $(SRC)/main.cpp -o $(LIB)/main.o
	ar rcs $(LIB)/libdecompress.a $(LIB)/huffman.o $(LIB)/main.o
	$(GNU) $(ARGS) $(OPTIMIZATIONS) $(LIB)/libdecompress.a -o $(LIB)/libdecompress.so

bin : lib
	mkdir -p $(BIN)
# $(GNU) $(OPTIMIZATIONS) $(SRC)/huffman.cpp $(SRC)/main.cpp -o $(BIN)/decompress
	$(GNU) $(ARGS) $(OPTIMIZATIONS) $(LIB)/libdecompress.a -o $(BIN)/decompress

clean :
	rm -f $(LIB)/huffman.o
	rm -f $(LIB)/main.o
	rm -f $(LIB)/libdecompress.a
	rm -f $(LIB)/libdecompress.so
	rm -rf $(LIB)
	rm -f $(BIN)/decompress
	rm -rf $(BIN)

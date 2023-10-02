GNU ?= g++
ARGS ?=
OPTIMIZATIONS ?= -O2

SRC = ./src
TESTS =
BIN = ./bin
LIB = ./lib

all : lib bin

bin :
	mkdir -p $(BIN)
# $(GNU) $(OPTIMIZATIONS) $(SRC)/huffman.cpp $(SRC)/main.cpp -o $(BIN)/decompress
	$(GNU) $(OPTIMIZATIONS) $(LIB)/libdecompress.a -o $(BIN)/decompress

lib :
	mkdir -p $(LIB)
	$(GNU) $(OPTIMIZATIONS) -c $(SRC)/huffman.cpp -o $(LIB)/huffman.o
	$(GNU) $(OPTIMIZATIONS) -c $(SRC)/main.cpp -o $(LIB)/main.o
	ar rcs $(LIB)/libdecompress.a $(LIB)/huffman.o $(LIB)/main.o

clean :
	rm -f $(LIB)/huffman.o
	rm -f $(LIB)/main.o
	rm -f $(LIB)/libdecompress.a
	rm -rf $(LIB)
	rm -f $(BIN)/decompress
	rm -rf $(BIN)

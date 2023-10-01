GNU ?= g++
ARGS ?=
OPTIMIZATIONS ?= -O2

SOURCE = ./src
TESTS =
BIN = ./bin

all :
	mkdir -p $(BIN)
	$(GNU) $(OPTIMIZATIONS) $(SOURCE)/huffman.cpp $(SOURCE)/main.cpp -o $(BIN)/decompress

clean :
	rm -f $(BIN)/decompress
	rm -r $(BIN)

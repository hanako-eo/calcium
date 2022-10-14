CFLAGS = -c -W -Wall -std=c17

SRC = src/calc.c

OBJ = $(SRC:.c=.o)
BIN = libcalc.a

.PHONY: all
all: build

build: 
	gcc $(SRC) $(CFLAGS) -o $(OBJ)
	ar rcs $(BIN) $(OBJ)

clean:
	$(RM) $(BIN) $(OBJ)
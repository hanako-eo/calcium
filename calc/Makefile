CFLAGS = -c -W -Wall -std=c17

LEXER_FILE = src/lexer.l
LEXER_C = src/lexer.c

PARSER_FILE = src/grammar.y
PARSER_C = src/grammar.c

SRC = $(LEXER_C) $(PARSER_C) src/calc.c

OBJ = $(SRC:.c=.o)
BIN = libcalc.a

.PHONY: all
all: build

.PHONY: build
build: $(OBJ)
	gcc -lm src/calc.c -o calculator
	ar rcs $(BIN) $?

%.o: %.c
	gcc $< $(CFLAGS) -o $@

$(LEXER_C): $(LEXER_FILE) $(PARSER_C)
	flex -o $(LEXER_C) $(LEXER_FILE)

$(PARSER_C): $(PARSER_FILE)
	bison -d -t $(PARSER_FILE) -o $(PARSER_C)

.PHONY: clean
clean:
	$(RM) $(BIN) $(OBJ)
	$(RM) $(LEXER_C) $(PARSER_C) $(PARSER_C:.c=.h)

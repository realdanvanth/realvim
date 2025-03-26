# Compiler and flags
CC = gcc
CFLAGS = -g -Wall -Wextra -Werror -I. -IRegression
LDFLAGS = -lm -lncurses

# Source files
# if any file or folder used, then mention here i.e folder/filename.c
SRCS = text.c keyboard/keyboard.c filehandler/file_handle.c modes/modes.c
OBJS = $(SRCS:.c=.o)

# Targets
all: build/text

build/text: $(OBJS)
	mkdir -p build
	$(CC) $(CFLAGS) -o $@ $(OBJS) $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJS) build/text
	rmdir build

.PHONY: all clean


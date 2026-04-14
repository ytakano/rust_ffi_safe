# Makefile for the FFI Safety Evaluation Catalog C test cases
#
# Builds all 26 test case source files into object files and archives
# them into a static library (libeval_catalog.a).
#
# Usage:
#   make          -- compile everything and build the library
#   make clean    -- remove all generated files
#
# Notes:
#   - Case 20 (20_concurrent_mutation.c) uses pthreads.  The compile step
#     does not require a special flag, but any executable or shared library
#     that links against libeval_catalog.a must add -lpthread (or -pthread)
#     to its own linker flags.
#   - Cases 07 and 24 contain intentional undefined behavior; their
#     compiler warnings are suppressed via #pragma GCC diagnostic inside
#     the source files.

CC      = gcc
CFLAGS  = -Wall -Wextra -std=c11
AR      = ar
ARFLAGS = rcs

SRCS = $(wildcard *.c)
OBJS = $(SRCS:.c=.o)
LIB  = libeval_catalog.a

.PHONY: all clean

all: $(LIB)

$(LIB): $(OBJS)
	$(AR) $(ARFLAGS) $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJS) $(LIB)

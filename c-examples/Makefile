CC = gcc
CFLAGS = -Wall -O4 -std=c99

# For compiling against the installation.
#RBJ_EQ_CFLAGS = `pkg-config --cflags rbj_eq`
#RBJ_EQ_LIBS = `pkg-config --libs rbj_eq`
TARGET = ../target/`sh ./target.sh`/release
RBJ_EQ_CFLAGS = -I$(TARGET)/
RBJ_EQ_LIBS = $(TARGET)/librbj_eq.a -lpthread -ldl

sweep: sweep.c ../target/release
	$(CC) $(CFLAGS) $(RBJ_EQ_CFLAGS) -o sweep \
            sweep.c $(RBJ_EQ_LIBS) -lm

../target/release:
	cargo +nightly cbuild --release

clean:
	-rm -f sweep

run: sweep
	./sweep 0.1 >/dev/null

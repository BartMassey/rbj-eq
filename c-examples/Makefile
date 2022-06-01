CC = gcc
CFLAGS = -Wall -O4 -std=c99

sweep: sweep.c
	$(CC) $(CFLAGS) `pkg-config --cflags rbj_eq` -o sweep \
            sweep.c `pkg-config --libs rbj_eq` -lm

clean:
	-rm -f sweep

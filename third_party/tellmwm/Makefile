# This software is provided with no  warranty whatsoever. You may use, modify
# and redistribute it as long as you give a credit to the original author and
# provide a link to the original source.
#
# https://fastestcode.org/emwm.html
# alx@fastestcode.org
#
# EMWM workspace manager control utility makefile
#
# $Id: Makefile,v 1.3 2026/03/15 13:55:33 alx Exp $


PREFIX = /usr
CFLAGS = -O2 -Wall -I/usr/local/include

OBJS = tellmwm.o wsmutil.o

all: tellmwm

tellmwm: $(OBJS)
	$(CC) -L/usr/local/lib -o $@ $(OBJS) -lX11

clean:
	-rm -f $(OBJS) tellmwm

install:
	install -m755 tellmwm $(PREFIX)/bin/tellmwm
	install -m644 tellmwm.1 $(PREFIX)/share/man/man1/tellmwm.1

uninstall:
	-rm $(PREFIX)/bin/tellmwm
	-rm $(PREFIX)/share/man/man1/tellmwm.1

tellmwm.o: tellmwm.c wsmutil.h
wsmutil.o: wsmutil.h

# makefile to fail if any command in pipe is failed.
SHELL = /bin/bash -o pipefail

GOLEM_IP = 0.0.0.0

# using gcc version 10.2.1

BASE    = arm-none-linux-gnueabihf

CC      = $(BASE)-gcc
LD      = $(BASE)-ld
STRIP   = $(BASE)-strip

ifeq ($(V),1)
	Q :=
else
	Q := @
endif

INCLUDE	= -I./
INCLUDE	+= -I./lib/libco
INCLUDE	+= -I./lib/miniz
INCLUDE	+= -I./lib/md5
INCLUDE += -I./lib/lzma
INCLUDE += -I./lib/libchdr/include
INCLUDE += -I./lib/flac/include
INCLUDE += -I./lib/flac/src/include
INCLUDE += -I./lib/bluetooth
INCLUDE += -I./lib/serial_server/library

PRJ = GoLEm_firmware
C_SRC =   $(wildcard *.c) \
          $(wildcard ./lib/miniz/*.c) \
          $(wildcard ./lib/md5/*.c) \
          $(wildcard ./lib/lzma/*.c) \
          $(wildcard ./lib/flac/src/*.c) \
          $(wildcard ./lib/libchdr/*.c) \
          lib/libco/arm.c

CPP_SRC = $(wildcard *.cpp) \
          $(wildcard ./lib/serial_server/library/*.cpp) \
          $(wildcard ./support/*/*.cpp)

IMG =     $(wildcard *.png)

IMLIB2_LIB  = -Llib/imlib2 -lfreetype -lbz2 -lpng16 -lz -lImlib2

OBJ	= $(C_SRC:.c=.c.o) $(CPP_SRC:.cpp=.cpp.o) $(IMG:.png=.png.o)
DEP	= $(C_SRC:.c=.c.d) $(CPP_SRC:.cpp=.cpp.d)

DFLAGS	= $(INCLUDE) -D_7ZIP_ST -DPACKAGE_VERSION=\"1.3.3\" -DFLAC_API_EXPORTS -DFLAC__HAS_OGG=0 -DHAVE_LROUND -DHAVE_STDINT_H -DHAVE_STDLIB_H -DHAVE_SYS_PARAM_H -DENABLE_64_BIT_WORDS=0 -D_FILE_OFFSET_BITS=64 -D_LARGEFILE64_SOURCE -DVDATE=\"`date +"%y%m%d"`\"
CFLAGS	= $(DFLAGS) -Wall -Wextra -Wno-strict-aliasing -Wno-stringop-overflow -Wno-stringop-truncation -Wno-format-truncation -Wno-psabi -Wno-restrict -c -O3
LFLAGS	= -Llib/sysroot/lib -ludev -l:libc-2.31.so -lstdc++ -ldl -lm -lSDL3 -lrt $(IMLIB2_LIB) -Llib/bluetooth -lbluetooth -lpthread -lmtdev -levdev -lsqlite3

ifeq ($(PROFILING),1)
	DFLAGS += -DPROFILING
endif

$(PRJ): $(OBJ) target/armv7-unknown-linux-gnueabihf/release/libgolem.a
	$(Q)$(info $@)
	$(Q)$(CC) -o $@ $+ $(LFLAGS)
	$(Q)cp $@ $@.elf
	$(Q)$(STRIP) $@

.PHONY: clean
clean:
	$(Q)rm -f *.elf *.map *.lst *.user *~ $(PRJ)
	$(Q)rm -rf obj DTAR* x64
	$(Q)find . \( -name '*.o' -o -name '*.d' -o -name '*.bak' -o -name '*.rej' -o -name '*.org' \) -exec rm -f {} \;

%.c.o: %.c
	$(Q)$(info $<)
	$(Q)$(CC) $(CFLAGS) -std=gnu99 -o $@ -c $< 2>&1 | sed -e 's/\(.[a-zA-Z]\+\):\([0-9]\+\):\([0-9]\+\):/\1(\2,\ \3):/g'

%.cpp.o: %.cpp
	$(Q)$(info $<)
	$(Q)$(CC) $(CFLAGS) -std=gnu++14 -Wno-class-memaccess -o $@ -c $< 2>&1 | sed -e 's/\(.[a-zA-Z]\+\):\([0-9]\+\):\([0-9]\+\):/\1(\2,\ \3):/g'

%.png.o: %.png
	$(Q)$(info $<)
	$(Q)$(LD) -r -b binary -o $@ $< 2>&1 | sed -e 's/\(.[a-zA-Z]\+\):\([0-9]\+\):\([0-9]\+\):/\1(\2,\ \3):/g'

ifneq ($(MAKECMDGOALS), clean)
-include $(DEP)
endif
%.c.d: %.c
	$(Q)$(CC) $(DFLAGS) -MM $< -MT $@ -MT $*.c.o -MF $@ 2>&1 | sed -e 's/\(.[a-zA-Z]\+\):\([0-9]\+\):\([0-9]\+\):/\1(\2,\ \3):/g'

%.cpp.d: %.cpp
	$(Q)$(CC) $(DFLAGS) -MM $< -MT $@ -MT $*.cpp.o -MF $@ 2>&1 | sed -e 's/\(.[a-zA-Z]\+\):\([0-9]\+\):\([0-9]\+\):/\1(\2,\ \3):/g'

# Ensure correct time stamp
main.cpp.o: $(filter-out main.cpp.o, $(OBJ))

# Custom rule to build rust libgolem.
#target/armv7-unknown-linux-gnueabihf/release/libgolem.a:
.PHONY: rust
rust:
	cargo build --release

# Custom rule to send the binary to the GoLEm.
.PHONY: send
send: $(PRJ)
	ssh root@$(GOLEM_IP) 'killall -9 MiSTer GoLEm_firmware'
	scp ./GoLEm_firmware root@$GOLEM_IP:/media/fat/GoLEm_firmware
	ssh root@$(GOLEM_IP) 'sync; PATH=/media/fat:$PATH; GoLEm_firmware'

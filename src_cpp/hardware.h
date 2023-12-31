#ifndef HARDWARE_H
#define HARDWARE_H

#include <inttypes.h>
#include <stdio.h>

extern "C" unsigned long GetTimer(unsigned long offset);
extern "C" unsigned long CheckTimer(unsigned long t);
extern "C" void WaitTimer(unsigned long time);

extern "C" void hexdump(void *data, uint16_t size, uint16_t offset = 0);

// minimig reset stuff
#define SPI_RST_USR         0x1
#define SPI_RST_CPU         0x2
#define SPI_CPU_HLT         0x4
extern "C" uint8_t rstval;

#endif // HARDWARE_H

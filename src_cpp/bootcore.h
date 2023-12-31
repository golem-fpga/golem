// bootcore.h
// 2019, Aitor Gomez Garcia (spark2k06@gmail.com)
// Thanks to Sorgelig and BBond007 for their help and advice in the development of this feature.

#ifndef __BOOTCORE_H__
#define __BOOTCORE_H__

extern "C" void bootcore_init(const char *path);

extern char bootcoretype[64];
extern int16_t btimeout;

#endif // __BOOTCORE_H__

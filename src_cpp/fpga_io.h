
#include <stdint.h>

#ifndef FPGAIO_H
#define FPGAIO_H

#define BUTTON_OSD  1
#define BUTTON_USR  2

//extern "C" int fpga_io_init();

extern "C" void fpga_spi_en(uint32_t mask, uint32_t en);
extern "C" uint16_t fpga_spi(uint16_t word);
uint16_t fpga_spi_fast(uint16_t word);

extern "C" void fpga_spi_fast_block_write(const uint16_t *buf, uint32_t length);
void fpga_spi_fast_block_read(uint16_t *buf, uint32_t length);
void fpga_spi_fast_block_write_8(const uint8_t *buf, uint32_t length);
void fpga_spi_fast_block_read_8(uint8_t *buf, uint32_t length);
void fpga_spi_fast_block_write_be(const uint16_t *buf, uint32_t length);
void fpga_spi_fast_block_read_be(uint16_t *buf, uint32_t length);

void fpga_set_led(uint32_t on);
int  fpga_get_buttons();
int fpga_get_io_type();

void fpga_core_reset(int reset);
void fpga_core_write(uint32_t offset, uint32_t value);
uint32_t fpga_core_read(uint32_t offset);
extern "C" int fpga_core_id();
extern "C" int is_fpga_ready(int quick);

extern "C" int fpga_get_fio_size();
extern "C" int fpga_get_io_version();

extern "C" int fpga_load_rbf(const char *name, const char *cfg = 0, const char *xml = 0);

extern "C" void reboot(int cold);
void app_restart(const char *path, const char *xml = 0);
char *getappname();

extern "C" void fpga_wait_to_reset();

#endif

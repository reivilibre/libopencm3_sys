// Comments mark omissions or groups of omissions; typically unsupported features by STM32F1.



///// Common  /////
#include <libopencm3/cm3/common.h>
#include <libopencm3/cm3/cortex.h>
#include <libopencm3/cm3/dwt.h>
// fpb
// itm
#include <libopencm3/cm3/memorymap.h>
#include <libopencm3/cm3/mpu.h>
#include <libopencm3/cm3/nvic.h>
#include <libopencm3/cm3/scb.h>
#include <libopencm3/cm3/scs.h>
#include <libopencm3/cm3/sync.h>
#include <libopencm3/cm3/systick.h>
// tpiu
#include <libopencm3/cm3/vector.h>

///// STM32F1 /////

#include <libopencm3/stm32/adc.h>
#include <libopencm3/stm32/can.h>
// #inclde <cec>
// #include <comparator>
#include <libopencm3/stm32/crc.h>
// ...
#include <libopencm3/stm32/dac.h>
// ...
#include <libopencm3/stm32/dma.h>
// ...
#include <libopencm3/stm32/exti.h>
#include <libopencm3/stm32/flash.h>
#include <libopencm3/stm32/fsmc.h>
#include <libopencm3/stm32/gpio.h>
// ...
#include <libopencm3/stm32/i2c.h>
#include <libopencm3/stm32/iwdg.h>
// ...
#include <libopencm3/stm32/memorymap.h>
#include <libopencm3/stm32/pwr.h>
// ...
#include <libopencm3/stm32/rcc.h>
// ...
#include <libopencm3/stm32/rtc.h>
// sdio not implemented upstream; not sure if included in this chip
#include <libopencm3/stm32/spi.h>
#include <libopencm3/stm32/st_usbfs.h>
// ...
#include <libopencm3/stm32/timer.h>
// tools <?>
// ...
#include <libopencm3/stm32/usart.h>
// wwdg marked as TODO upstream.


/////  USB  /////
// msc depends on the below file.
#include <libopencm3/usb/usbd.h>

#include <libopencm3/usb/audio.h>
#include <libopencm3/usb/cdc.h>
#include <libopencm3/usb/dfu.h>
#include <libopencm3/usb/hid.h>
#include <libopencm3/usb/midi.h>
#include <libopencm3/usb/msc.h>
// usbd moved higher
#include <libopencm3/usb/usbstd.h>

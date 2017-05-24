#ifndef CUSTOM_BOARD_H
#define CUSTOM_BOARD_H

#include "nrf_gpio.h"

#define LED_START      18
#define LED_1          18
#define LED_2          19
#define LED_3          20
#define LED_4          21
#define LED_5          22
#define LED_STOP       22

#define LEDS_ACTIVE_STATE 1

#define LEDS_NUMBER    5
#define LEDS_LIST { LED_1, LED_2, LED_3, LED_4, LED_5 }

#endif

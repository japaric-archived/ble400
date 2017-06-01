// Make sure that nrf doesn't think we are compiling for a PC
#if defined(_WIN32)
#undef _WIN32
#endif
#if defined(__unix)
#undef __unix
#endif
#if defined(__APPLE__)
#undef __APPLE__
#endif

#include "app_timer.h"
#include "app_uart.h"
#include "app_util_platform.h"
#include "ble_advdata.h"
#include "ble_advertising.h"
#include "ble_conn_params.h"
#include "ble_gap.h"
#include "ble_nus.h"
#include "boards.h"
#include "bsp.h"
#include "nrf_gpio.h"
#include "shims.h"
#include "softdevice_handler.h"

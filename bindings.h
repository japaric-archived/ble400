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
#include "boards.h"
#include "nrf_gpio.h"
#include "shims.h"

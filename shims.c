#include "nrf_delay.h"
#include "nrf_sdm.h"

#include "custom_board.h"
#include "shims.h"

void _nrf_delay_ms(uint32_t number_of_ms) { nrf_delay_ms(number_of_ms); }

nrf_clock_lf_cfg_t _NRF_CLOCK_LFCLKSRC() {
  return (nrf_clock_lf_cfg_t)NRF_CLOCK_LFCLKSRC;
}

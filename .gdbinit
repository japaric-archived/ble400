target remote :3333
monitor arm semihosting enable
load
tbreak Reset_Handler
monitor reset halt
continue

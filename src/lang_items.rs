#[lang = "start"]
extern "C" fn start(
    main: fn(),
    _argc: isize,
    _argv: *const *const u8,
) -> isize {
    main();

    0
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(
    args: ::core::fmt::Arguments,
    file: &'static str,
    line: u32,
) -> ! {
    hprint!("panicked at '");
    ::cortex_m_semihosting::io::write_fmt(args);
    hprintln!("', {}:{}", file, line);

    ::bkpt();

    loop {}
}

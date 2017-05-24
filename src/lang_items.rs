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
extern "C" fn panic_fmt() -> ! {
    loop {}
}

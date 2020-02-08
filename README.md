# cmsis_rtos2_rs

A rust wrapper crate for the [CMSIS-RTOS2 API](http://www.keil.com/pack/doc/CMSIS_Dev/RTOS2/html/index.html), 
which should be provided by another library,
and linked into the consuming library or application.

Typically the target architecture is provided by the build consuming this crate. 
You can test this with eg:
```
cargo build --target thumbv7em-none-eabihf
```
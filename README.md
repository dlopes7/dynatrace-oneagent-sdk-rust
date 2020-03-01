# Dynatrace OneAgent SDK Tests

This is just a test instrumenting Rust with the Dynatrace OneAgent SDK for C/C++


### Usage
Set the environment variable `ONESDK_LIB_DIR` to the lib folder of the ONESDK in your platform.

Example for linux x64:
`export ONESDK_LIB_DIR=/projects/c/OneAgent-SDK-for-C/lib/linux-x86_64`

Then `cargo run` or `cargo build`

The example just starts and ends a 2s purepath (Custom Service tracer) 10 times. 


### Purepaths

![Purepaths](https://i.imgur.com/MPgFnja.png)
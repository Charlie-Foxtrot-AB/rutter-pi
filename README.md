# rutter-pi

## Build
Clone [flutter-pi](https://github.com/ardera/flutter-pi/tree/master) and the `engine-binares` branch mentioned in the README on `master`. Remember to also copy the files mentioned in `engine-binares/README` to the correct location on the Rpi.  
The build steps after `flutter build bundle` has been scripted in `build_snapshot.sh` and `build_so.sh` correspondingly.  

You will need to modify the scripts to match your paths for `engine-binaries` and `flutter`.  

### System dependencies
We need the `arm-linux-gnueabihf-gcc` (the exact package name may differ depending on distro) dependency in order to link the rust binary later on. Install this using what ever package manager your distro might offer. 

### Rust specific
Flutter will interoperate with rust as a library. Therefore we want to build and compile a `.so` file.  
Create a new project using `cargo <project name> --lib`
then in order to compile it for the Rpi we need to install the correct target. This is done by running: `rustup component add rust-std-armv7-unknown-linux-gnueabihf`. 
When building we would like to not have to specify the target everytime. In order to do that we run `mkdir .cargo && touch .cargo/config.toml`. In this file we now place:  
```toml
[build]
target = "armv7-unknown-linux-gnueabihf"

[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```
This enables us to just run `cargo build --release` in order to build.

### Build the complete app
```bash
# standing in the rust project dir
$ cargo build --release
# standing in the flutter project dir
$ flutter channel stable
$ flutter upgrade
$ flutter bundle build
$ ./build_snapshot.sh
$ ./build_so.sh
$ rsync -a --info=progress2 ./build/flutter_assets pi@raspberrypi:/home/pi/flutter_<project name>_assets
$ scp build/app.so pi@raspberrypi:/home/pi/flutter_<project name>_assets/app.so
```
Make sure that the rust `.so` file is in the same relative path as it was in you dart code (relative to where you run flutter-pi).  
Once this is done and you've built `flutter-pi` on the Rpi itself, you should be able to run the app using `flutter-pi /path/to/flutter_<project name>_assets/flutter_assets`

#!/bin/bash
rsync -a --info=progress2 ./build/flutter_assets/ pi@raspberrypi:/home/pi/flutter_hello_world_assets && \
	scp ./build/app.so pi@raspberrypi:/home/pi/flutter_hello_world_assets/app.so


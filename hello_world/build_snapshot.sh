#!/bin/bash

~/programming/clones/flutter//bin/cache/dart-sdk/bin/dart \
	~/programming/clones/flutter/bin/cache/dart-sdk/bin/snapshots/frontend_server.dart.snapshot \
	--sdk-root ~/programming/clones/flutter/bin/cache/artifacts/engine/common/flutter_patched_sdk_product \
	--target=flutter \
	--aot \
	--tfa \
	-Ddart.vm.product=true \
	--packages .packages \
	--output-dill build/kernel_snapshot.dill \
	--verbose \
	--depfile build/kernel_snapshot.d \
	package:hello_world/main.dart

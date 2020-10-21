#!/bin/sh

~/engine-binaries/gen_snapshot_linux_x64 \
  --causal_async_stacks \
  --deterministic \
  --snapshot_kind=app-aot-elf \
  --elf=build/app.so \
  --strip \
  --sim_use_hardfp \
  --no-use-integer-division \
  build/kernel_snapshot.dill


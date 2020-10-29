#!/bin/bash
flutter clean && \
	flutter build bundle --release && \
	./build_snapshot.sh && \
	./build_so.sh \

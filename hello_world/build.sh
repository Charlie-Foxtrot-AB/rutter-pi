#!/bin/bash
flutter clean && \
	flutter build bundle && \
	./build_snapshot.sh && \
	./build_so.sh \

#!/bin/sh
SDK=C:/tfm2mods/sdk_058/mod-sdk
DEPS=$SDK/deps
NAT=$SDK/native
SRC=$1
OUT=C:/Users/jungs/AppData/Local/Temp/tfm2_spanprobe
pick(){ ls $DEPS/lib$1-*.rlib | head -1; }
"$USERPROFILE/.cargo/bin/rustup.exe" run nightly-2026-05-24 rustc --edition 2021 \
  -L dependency=$DEPS -L native=$NAT \
  --extern mod_api=$(pick mod_api) --extern engine_ui=$(pick engine_ui) \
  --extern engine_core=$(pick engine_core) --extern game_core=$(pick game_core) \
  --extern game_view=$(pick game_view) --extern common=$(pick common) --extern game_ai=$(pick game_ai) \
  --extern bumpalo=$DEPS/libbumpalo-dafef1f270bdb02f.rlib \
  --extern rand=$DEPS/librand-e2a5dd20f067a3a7.rlib \
  -C lto=fat -C codegen-units=1 -C opt-level=1 \
  "$SRC" --out-dir "$OUT" 2>&1

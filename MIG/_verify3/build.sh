#!/bin/sh
# 오라클 프로브 빌드 — `sh build.sh <프로브.rs>` → %TEMP%\tfm2_spanprobe\<이름>.exe
# _verify2/A/A2_build.sh 와 같고 **serde_json 만 추가**했다(실전 game_setting 파일 로드용).
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
  --extern serde_json=$DEPS/libserde_json-6dcbdfea7162b679.rlib \
  -C lto=fat -C codegen-units=1 -C opt-level=1 \
  "$SRC" --out-dir "$OUT" 2>&1

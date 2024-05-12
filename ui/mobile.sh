#!/bin/bash

export ANDROID_HOME=$HOME/Library/Android/Sdk
export ANDROID_NDK_HOME=$HOME/Library/Android/Sdk/ndk/27.0.11718014
export NDK_HOME=$ANDROID_NDK_HOME

INPUT_TARGET=$1
ACTION=$2
if [ "$INPUT_TARGET" == "android" ];
then 
    export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64
    export TARGET=aarch64-linux-android
    export API=33

    export AR=$TOOLCHAIN/bin/llvm-ar
    export CC=$TOOLCHAIN/bin/$TARGET$API-clang
    export AS=$CC
    export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
    export LD=$TOOLCHAIN/bin/ld
    export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
    export STRIP=$TOOLCHAIN/bin/llvm-strip

    export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
    export PATH=$PATH:$TOOLCHAIN/bin
    echo "Path updated for Android"
    cargo android $ACTION
else
    unset $TOOLCHAIN
    unset $TARGET
    unset $API
    unset $AR
    unset $CC
    unset $AS
    unset $CXX
    unset $LD
    unset $RANLIB
    unset $STRIP
    echo "Path unset for macs"
fi 


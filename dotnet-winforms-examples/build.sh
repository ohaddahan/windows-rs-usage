#!/usr/bin/env bash

export MSBUILD='/c/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/MSBuild/Current/Bin/MSBuild.exe'

"${MSBUILD}" \
-t:restore,build \
//p:Platform="Any CPU" \
//p:TargetFrameworkVersion=v6.0 \
//p:Configuration=Release \
//p:RestorePackagesConfig=true \
//p:PublishSingleFile=true \
//p:TargetFramework="net6.0-windows" \
//p:RuntimeIdentifier="win-x64"

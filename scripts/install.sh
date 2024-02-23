#!/usr/bin/env bash
set -euo pipefail

# Wrap everything in a function so that a truncated script
# does not have the chance to cause issues.
__wrap__() {

NAME="mk"
REPO=martinwalsh/mk

# allow overriding the version
VERSION=${MK_VERSION:-latest}

PLATFORM=`uname -s`
ARCH=`uname -m`

if [[ $PLATFORM == "Darwin" ]]; then
  PLATFORM="macos"
elif [[ $PLATFORM == "Linux" ]]; then
  PLATFORM="linux"
fi

if [[ $ARCH == armv8* ]] || [[ $ARCH == arm64* ]] || [[ $ARCH == aarch64* ]]; then
  ARCH="aarch64"
elif [[ $ARCH == i686* ]]; then
  ARCH="x86"
fi

BINARY="${NAME}-${ARCH}-${PLATFORM}"

# Oddly enough GitHub has different URLs for latest vs specific version
if [[ $VERSION == "latest" ]]; then
  DOWNLOAD_URL=https://github.com/${REPO}/releases/latest/download/${BINARY}.gz
else
  DOWNLOAD_URL=https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}.gz
fi

echo "This script will automatically download and install ${NAME} (${VERSION}) for you."
if [ "x$(id -u)" == "x0" ]; then
  echo "warning: this script is running as root.  This is dangerous and unnecessary!"
fi

if ! hash curl 2> /dev/null; then
  echo "error: you do not have 'curl' installed which is required for this script."
  exit 1
fi

if ! hash gunzip 2> /dev/null; then
  echo "error: you do not have 'gunzip' installed which is required for this script."
  exit 1
fi

TEMP_FILE=`mktemp "${TMPDIR:-/tmp}/.${NAME}install.XXXXXXXX"`
TEMP_FILE_GZ="${TEMP_FILE}.gz"

cleanup() {
  rm -f "$TEMP_FILE"
  rm -f "$TEMP_FILE_GZ"
}

trap cleanup EXIT
HTTP_CODE=$(curl -SL --progress-bar "$DOWNLOAD_URL" --output "$TEMP_FILE_GZ" --write-out "%{http_code}")
if [[ ${HTTP_CODE} -lt 200 || ${HTTP_CODE} -gt 299 ]]; then
  echo "error: platform ${PLATFORM} (${ARCH}) is unsupported."
  exit 1
fi

rm -f "$TEMP_FILE"
gunzip "$TEMP_FILE_GZ"
chmod +x "$TEMP_FILE"

# Detect when the file cannot be executed due to NOEXEC /tmp.  Taken from rustup
# https://github.com/rust-lang/rustup/blob/87fa15d13e3778733d5d66058e5de4309c27317b/rustup-init.sh#L158-L159
if [ ! -x "$TEMP_FILE" ]; then
  printf '%s\n' "Cannot execute $TEMP_FILE (likely because of mounting /tmp as noexec)." 1>&2
  printf '%s\n' "Please copy the file to a location where you can execute binaries and run it manually." 1>&2
  exit 1
fi

mkdir -p "${HOME/.local/bin}"
cp -i ${TEMP_FILE} "${HOME/.local/bin}/${NAME}"

echo "Successfully installed ${NAME} (${VERSION}) to ${HOME/.local/bin}/${NAME}"
echo "Please add ${HOME/.local/bin} to your PATH to use ${NAME}."

}; __wrap__

# This file was borrowed from the `rye` project with gratitude,
# and slightly modified. The `rye` project's license is included below.

# MIT License
#
# Copyright (c) 2023, Armin Ronacher
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

#!/usr/bin/env python
import argparse
import hashlib
import sys

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Hash a file")
    parser.add_argument(
        "--list",
        action="store_true",
        help="List the available hash algorithms",
    )
    if "--list" not in sys.argv:
        parser.add_argument("file", help="The file to hash")
        parser.add_argument("algorithm", help="The hash algorithm to use")
    args = parser.parse_args()

    if args.list:
        for algorithm in sorted(hashlib.algorithms_available):
            print(algorithm)
        sys.exit(0)

    hasher = getattr(hashlib, args.algorithm)()
    with open(args.file, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hasher.update(chunk)
    print(hasher.hexdigest())

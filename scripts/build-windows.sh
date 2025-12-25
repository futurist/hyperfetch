#!/bin/bash
# Build script for Windows platform

echo "Building for Windows with optimizations..."

# Set target for Windows
export CARGO_TARGET=x86_64-pc-windows-msvc

# Set optimization flags for smaller binary size
export RUSTFLAGS="-C opt-level=z -C codegen-units=1 -C strip=symbols"

# Build the release version with optimizations
cargo build --release --target $CARGO_TARGET

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Windows build completed successfully!"
    echo "Binary location: target/$CARGO_TARGET/release/fetch.exe"
    
    # Try to use strip if available (GNU binutils for Windows)
    if command -v strip >/dev/null 2>&1; then
        echo "Stripping debug symbols..."
        strip "target/$CARGO_TARGET/release/fetch.exe"
        echo "Binary stripped successfully!"
    else
        echo "Note: Install 'strip' from GNU binutils for Windows for additional size reduction"
        echo "Download from: https://sourceforge.net/projects/mingw/files/MinGW/Base/binutils/"
    fi
    
    # Show binary size
    echo "Final binary size:"
    ls -lh "target/$CARGO_TARGET/release/fetch.exe"
else
    echo "Windows build failed!"
    exit 1
fi
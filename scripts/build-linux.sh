#!/bin/bash
# Build script for Linux platform

echo "Building for Linux with optimizations..."

# Set target for Linux (using GNU libc for maximum compatibility)
export CARGO_TARGET=x86_64-unknown-linux-gnu

# Set optimization flags for smaller binary size
export RUSTFLAGS="-C opt-level=z -C codegen-units=1 -C strip=symbols"

# Build the release version with optimizations
cargo build --release --target $CARGO_TARGET

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Linux build completed successfully!"
    echo "Binary location: target/$CARGO_TARGET/release/fetch"
    
    # Additional strip for maximum size reduction
    if command -v strip >/dev/null 2>&1; then
        echo "Stripping debug symbols..."
        strip "target/$CARGO_TARGET/release/fetch"
        echo "Binary stripped successfully!"
    fi
    
    # Show binary size
    echo "Final binary size:"
    ls -lh "target/$CARGO_TARGET/release/fetch"
else
    echo "Linux build failed!"
    exit 1
fi
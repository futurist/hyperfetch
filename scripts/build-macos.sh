#!/bin/bash
# Build script for macOS platform (Intel + ARM)

echo "Building for macOS..."

# Build for both Intel (x86_64) and ARM (aarch64) architectures
ARCHS=("x86_64-apple-darwin" "aarch64-apple-darwin")

for arch in "${ARCHS[@]}"; do
    echo "Building for architecture: $arch"
    
    # Set optimization flags for smaller binary size
    export RUSTFLAGS="-C opt-level=z -C codegen-units=1 -C strip=symbols"
    
    # Build the release version for specific architecture
    cargo build --release --target $arch
    
    # Check if build was successful
    if [ $? -eq 0 ]; then
        echo "Build for $arch completed successfully!"
        echo "Binary location: target/$arch/release/fetch"
        
        # Strip debug symbols to reduce binary size
        echo "Stripping debug symbols for $arch..."
        if command -v strip >/dev/null 2>&1; then
            strip "target/$arch/release/fetch"
            echo "Binary stripped successfully!"
        fi
        
        # Show binary size
        echo "Final binary size for $arch:"
        ls -lh "target/$arch/release/fetch"
    else
        echo "Build for $arch failed!"
        exit 1
    fi
    
    echo ""
done

echo "macOS builds completed for both Intel and ARM architectures!"
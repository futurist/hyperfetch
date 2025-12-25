#!/bin/bash
# Main build script that builds for all platforms

echo "=== Building fetch for all platforms ==="

# Function to run a build script and check for errors
run_build_script() {
    local script_name=$1
    local platform=$2
    
    echo ""
    echo "=== Building for $platform ==="
    
    if [ -f "scripts/$script_name" ]; then
        chmod +x "scripts/$script_name"
        ./scripts/$script_name
        if [ $? -ne 0 ]; then
            echo "Error: $platform build failed!"
            exit 1
        fi
    else
        echo "Error: Build script for $platform not found!"
        exit 1
    fi
}

# Build for all platforms
run_build_script "build-linux.sh" "Linux"
run_build_script "build-macos.sh" "macOS"
run_build_script "build-windows.sh" "Windows"

echo ""
echo "=== All builds completed successfully! ==="
echo ""
echo "Binary locations:"
echo "- Linux: target/x86_64-unknown-linux-gnu/release/fetch"
echo "- macOS Intel: target/x86_64-apple-darwin/release/fetch"
echo "- macOS ARM: target/aarch64-apple-darwin/release/fetch"
echo "- Windows: target/x86_64-pc-windows-msvc/release/fetch.exe"

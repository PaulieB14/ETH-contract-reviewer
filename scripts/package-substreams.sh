#!/bin/bash

# Exit on error
set -e

# Check if substreams CLI is installed
if ! command -v substreams &> /dev/null; then
    echo "Warning: substreams CLI is not installed."
    echo "You can install it by following the instructions at https://docs.substreams.dev/getting-started"
    echo "For now, we'll create a placeholder .spkg file for development purposes."
    
    # Create a placeholder .spkg file
    PACKAGE_NAME="contract_reviewer"
    PACKAGE_VERSION="v0.1.0"
    
    # Create an empty file
    touch "${PACKAGE_NAME}-${PACKAGE_VERSION}.spkg"
    
    echo "Created placeholder: ${PACKAGE_NAME}-${PACKAGE_VERSION}.spkg"
    echo "Note: This is just a placeholder file. You'll need to install the substreams CLI"
    echo "and build the actual package before deploying to production."
    exit 0
fi

# Check if the substreams.yaml file exists
if [ ! -f "./substreams.yaml" ]; then
    echo "Error: substreams.yaml file not found in the current directory."
    exit 1
fi

# Get package name and version from substreams.yaml
PACKAGE_NAME=$(grep -A 1 "package:" substreams.yaml | grep "name:" | cut -d ":" -f 2 | tr -d ' ')
PACKAGE_VERSION=$(grep -A 2 "package:" substreams.yaml | grep "version:" | cut -d ":" -f 2 | tr -d ' ')

if [ -z "$PACKAGE_NAME" ] || [ -z "$PACKAGE_VERSION" ]; then
    echo "Error: Could not extract package name or version from substreams.yaml"
    exit 1
fi

echo "Packaging substreams: $PACKAGE_NAME-$PACKAGE_VERSION"

# Create the package
substreams pack -o "${PACKAGE_NAME}-${PACKAGE_VERSION}.spkg"

echo "Package created: ${PACKAGE_NAME}-${PACKAGE_VERSION}.spkg"
echo "You can now use this package in your subgraph.yaml file."

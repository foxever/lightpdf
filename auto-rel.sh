#!/bin/bash

set -e

TAG_NAME="v0.0.1"

echo "======================================"
echo "  lingpdf Release Script"
echo "======================================"
echo ""

echo "Step 1: Checking for existing tags..."

# Delete local tag if exists
if git tag -l | grep -q "^${TAG_NAME}$"; then
    echo "  Found local tag: ${TAG_NAME}"
    git tag -d "${TAG_NAME}"
    echo "  ✓ Deleted local tag: ${TAG_NAME}"
else
    echo "  No local tag found: ${TAG_NAME}"
fi

# Delete remote tag if exists
if git ls-remote --tags origin | grep -q "refs/tags/${TAG_NAME}$"; then
    echo "  Found remote tag: ${TAG_NAME}"
    git push origin --delete "${TAG_NAME}"
    echo "  ✓ Deleted remote tag: ${TAG_NAME}"
else
    echo "  No remote tag found: ${TAG_NAME}"
fi

echo ""
echo "Step 2: Creating new tag..."

# Create new tag
git tag -a "${TAG_NAME}" -m "Release ${TAG_NAME}"
echo "  ✓ Created tag: ${TAG_NAME}"

echo ""
echo "Step 3: Pushing to remote..."

# Push tag to remote
git push origin "${TAG_NAME}"
echo "  ✓ Pushed tag to remote: ${TAG_NAME}"

echo ""
echo "======================================"
echo "  Done!"
echo "======================================"
echo ""
echo "GitHub Actions will automatically build and create a release."
echo "Watch the progress at: https://github.com/<your-repo>/actions"
echo ""

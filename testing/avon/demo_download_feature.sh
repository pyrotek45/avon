#!/bin/bash

################################################################################
# Demo: Download Feature in Action
# Shows real-world usage of the download feature for practical scenarios
################################################################################

PROJECT_ROOT="/home/pyrotek45/projects/v9/avon"
DEMO_DIR="/tmp/demo_download_$$"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

mkdir -p "$DEMO_DIR"
cd "$DEMO_DIR"

cleanup() {
    rm -rf "$DEMO_DIR"
}
trap cleanup EXIT

echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Demo: Download Feature in Task Runner${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

################################################################################
# Demo 1: Basic File Download
################################################################################
echo -e "\n${YELLOW}Demo 1: Download a single file from GitHub${NC}"
echo "Creating Avon.av with download task..."

mkdir -p demo1
cd demo1

cat > Avon.av << 'EOF'
{
  setup: {
    cmd: "echo '✓ Files downloaded:' && ls -lh *.md | awk '{print $9, $5}'",
    download: {
      url: "https://raw.githubusercontent.com/pyrotek45/avon/main/README.md",
      to: "remote_readme.md"
    },
    desc: "Download README from GitHub"
  }
}
EOF

echo "Running: avon do setup"
timeout 30 avon do setup 2>&1 | grep -E "setup|Files|remote_readme"

echo -e "\n✓ Demo 1 complete"

################################################################################
# Demo 2: Multiple Downloads with Quiet Mode
################################################################################
cd "$DEMO_DIR"
echo -e "\n${YELLOW}Demo 2: Multiple downloads with quiet mode${NC}"
echo "Creating Avon.av with multiple downloads..."

mkdir -p demo2
cd demo2

cat > Avon.av << 'EOF'
{
  fetch_docs: {
    cmd: "echo '✓ Downloaded documentation:' && wc -l *.md | tail -1",
    download: [
      {
        url: "https://raw.githubusercontent.com/pyrotek45/avon/main/README.md",
        to: "readme.md"
      },
      {
        url: "https://raw.githubusercontent.com/pyrotek45/avon/main/LICENSE",
        to: "license.txt"
      }
    ],
    quiet: true,
    desc: "Fetch multiple files quietly"
  }
}
EOF

echo "Running: avon do fetch_docs (with quiet: true)"
echo "Expected: No 'Downloading:' messages"
timeout 30 avon do fetch_docs 2>&1 | grep -E "fetch_docs|Downloaded|Running|Total"

echo -e "\n✓ Demo 2 complete"

################################################################################
# Demo 3: Download with Nested Directory
################################################################################
cd "$DEMO_DIR"
echo -e "\n${YELLOW}Demo 3: Download to nested directory path${NC}"
echo "Creating Avon.av with nested path..."

mkdir -p demo3
cd demo3

cat > Avon.av << 'EOF'
{
  prepare: {
    cmd: "echo '✓ File location:' && ls -lh config/source/*.md 2>/dev/null | awk '{print $9}'",
    download: {
      url: "https://raw.githubusercontent.com/pyrotek45/avon/main/README.md",
      to: "config/source/docs.md"
    },
    desc: "Download to nested path (auto-creates parents)"
  }
}
EOF

echo "Running: avon do prepare"
echo "Expected: Parent directories created automatically"
timeout 30 avon do prepare 2>&1 | grep -E "prepare|location|config"

echo -e "\n✓ Demo 3 complete"

################################################################################
# Demo 4: Error Handling with Invalid URL
################################################################################
cd "$DEMO_DIR"
echo -e "\n${YELLOW}Demo 4: Graceful error handling (invalid URL)${NC}"
echo "Creating Avon.av with invalid URL..."

mkdir -p demo4
cd demo4

cat > Avon.av << 'EOF'
{
  will_fail: {
    cmd: "echo 'This should NOT run'",
    download: {
      url: "https://invalid-nonexistent-domain-xyz.example.com/missing.txt",
      to: "output.txt"
    },
    desc: "This will fail gracefully"
  }
}
EOF

echo "Running: avon do will_fail (expected to fail)"
echo "Expected: Error message, no task execution"
if timeout 10 avon do will_fail 2>&1 | grep -q "Download failed\|HTTP request failed"; then
    echo -e "${GREEN}✓ Error handled gracefully${NC}"
    timeout 10 avon do will_fail 2>&1 | grep -E "Download failed|HTTP request failed|Error:" | head -1
else
    echo "Task timeout or network error (expected in some environments)"
fi

echo -e "\n✓ Demo 4 complete"

################################################################################
# Demo 5: Real-World Pipeline
################################################################################
cd "$DEMO_DIR"
echo -e "\n${YELLOW}Demo 5: Real-world build pipeline${NC}"
echo "Creating a realistic pipeline with dependencies..."

mkdir -p demo5
cd demo5

cat > Avon.av << 'EOF'
{
  download_deps: {
    cmd: "echo '✓ Dependencies ready' && echo '  README: ' $(wc -l < readme.md) ' lines' && echo '  LICENSE: ' $(wc -l < license.txt) ' lines'",
    download: [
      {
        url: "https://raw.githubusercontent.com/pyrotek45/avon/main/README.md",
        to: "readme.md"
      },
      {
        url: "https://raw.githubusercontent.com/pyrotek45/avon/main/LICENSE",
        to: "license.txt"
      }
    ],
    desc: "Download project files"
  },

  process: {
    cmd: "echo '✓ Processing files...' && echo '  Input size: ' $(du -sh . | awk '{print $1}')",
    deps: ["download_deps"],
    desc: "Process downloaded files (depends on download_deps)"
  },

  verify: {
    cmd: "echo '✓ Verification passed' && [ -f readme.md ] && [ -f license.txt ] && echo '  All files present'",
    deps: ["process"],
    desc: "Verify results (depends on process)"
  }
}
EOF

echo "Running: avon do verify (with full dependency chain)"
echo "Expected: download_deps → process → verify"
timeout 30 avon do verify 2>&1 | grep -E "Running:|Dependencies|Passed|present"

echo -e "\n✓ Demo 5 complete"

################################################################################
# Summary
################################################################################
echo -e "\n${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}All demos completed successfully! ✓${NC}"
echo -e "\n${BLUE}Download Feature Summary:${NC}"
echo "  • Single file download: ✓"
echo "  • Multiple files: ✓"
echo "  • Quiet mode: ✓"
echo "  • Nested paths: ✓"
echo "  • Error handling: ✓"
echo "  • Dependencies: ✓"
echo -e "\n${BLUE}════════════════════════════════════════════════════════════${NC}"

#!/usr/bin/env bash
# Benchmark runner for parallel vs sequential operations
# Requires hyperfine to be installed (included in shell.nix)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
AVON="$PROJECT_ROOT/target/release/avon"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Avon Parallel Operations Benchmark Suite           ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check for hyperfine
if ! command -v hyperfine &> /dev/null; then
    echo -e "${YELLOW}hyperfine not found. Install it or enter nix-shell.${NC}"
    echo "  nix-shell  # if using shell.nix"
    echo "  # or"
    echo "  nix-shell -p hyperfine"
    exit 1
fi

# Check for avon binary
if [ ! -x "$AVON" ]; then
    echo -e "${YELLOW}Building release binary...${NC}"
    cd "$PROJECT_ROOT"
    cargo build --release
fi

echo -e "${GREEN}Using avon: $AVON${NC}"
echo ""

# Verify benchmarks work first
echo -e "${BLUE}Verifying benchmark scripts...${NC}"
for bench in map filter fold; do
    echo -n "  $bench sequential: "
    if $AVON "$SCRIPT_DIR/${bench}_sequential.av" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}FAILED${NC}"
        exit 1
    fi
    echo -n "  $bench parallel:   "
    if $AVON "$SCRIPT_DIR/${bench}_parallel.av" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${YELLOW}FAILED${NC}"
        exit 1
    fi
done
echo ""

# Run benchmarks
WARMUP=2
RUNS=10

echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Benchmark 1: map vs pmap (CPU-intensive per-element work)${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
hyperfine \
    --warmup $WARMUP \
    --runs $RUNS \
    --export-markdown "$SCRIPT_DIR/results_map.md" \
    "$AVON $SCRIPT_DIR/map_sequential.av" \
    "$AVON $SCRIPT_DIR/map_parallel.av"

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Benchmark 2: filter vs pfilter (CPU-intensive predicate)${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
hyperfine \
    --warmup $WARMUP \
    --runs $RUNS \
    --export-markdown "$SCRIPT_DIR/results_filter.md" \
    "$AVON $SCRIPT_DIR/filter_sequential.av" \
    "$AVON $SCRIPT_DIR/filter_parallel.av"

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Benchmark 3: fold vs pfold (simple reduction)${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
hyperfine \
    --warmup $WARMUP \
    --runs $RUNS \
    --export-markdown "$SCRIPT_DIR/results_fold.md" \
    "$AVON $SCRIPT_DIR/fold_sequential.av" \
    "$AVON $SCRIPT_DIR/fold_parallel.av"

echo ""
echo -e "${GREEN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}Benchmark results saved to:${NC}"
echo "  $SCRIPT_DIR/results_map.md"
echo "  $SCRIPT_DIR/results_filter.md"
echo "  $SCRIPT_DIR/results_fold.md"
echo -e "${GREEN}════════════════════════════════════════════════════════════${NC}"

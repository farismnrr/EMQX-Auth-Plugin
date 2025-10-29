#!/bin/bash
set -euo pipefail

# Default ROCKSDB_PATH to the script directory if not provided
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
: "${ROCKSDB_PATH:=$SCRIPT_DIR}"

echo "[*] Checking RocksDB connection..."
if [ ! -d "$ROCKSDB_PATH" ]; then
  echo "❌ RocksDB directory not found at $ROCKSDB_PATH"
  echo "Hint: set ROCKSDB_PATH to the RocksDB database directory before running the script."
  exit 1
fi

# Find available ldb tool
if command -v rocksdb_ldb >/dev/null 2>&1; then
  LDB_CMD=rocksdb_ldb
elif command -v ldb >/dev/null 2>&1; then
  LDB_CMD=ldb
else
  echo "❌ Could not find 'rocksdb_ldb' or 'ldb' in PATH."
  echo "Hint: this container/runtime expects RocksDB artifacts mounted to /usr/local."
  echo
  echo "Quick options to provide the tools without rebuilding RocksDB every time:" 
  echo "  1) If you have already built a full image 'my-rocksdb:8.11.3', extract artifacts once:" 
  echo "       mkdir -p ./rocksdb/artifacts && \\"
  echo "       docker run --rm my-rocksdb:8.11.3 bash -lc 'tar -C /usr/local -c librocksdb* bin/ldb' \\"
  echo "         | tar -C ./rocksdb/artifacts -xvf -"
  echo "     This will populate ./rocksdb/artifacts with ldb and librocksdb.so files."
  echo
  echo "  2) Or (fast) mount a local prebuilt ldb and libs into ./rocksdb/artifacts and restart the service."
  echo
  echo "  3) Or run the check once inside the heavy image and copy files out:\\"
  echo "       docker run --rm -v \$(pwd)/rocksdb/artifacts:/out -w / \\"
  echo "         my-rocksdb:8.11.3 bash -lc 'cp /usr/local/bin/ldb /out/ || true; cp /usr/local/lib/librocksdb* /out/ || true'"
  echo
  echo "After putting ldb and librocksdb*.so into ./rocksdb/artifacts, bring the compose service up and the check will work without rebuilding RocksDB."
  exit 1
fi

# Use a unique test key to avoid collisions
TEST_KEY="check_connection_test_key_$$"
TEST_VALUE="check_connection_test_value"

echo "[*] Using DB: $ROCKSDB_PATH (tool: $LDB_CMD)"

# Try write
if ! "$LDB_CMD" --db="$ROCKSDB_PATH" put "$TEST_KEY" "$TEST_VALUE"; then
  echo "❌ Failed to write to RocksDB using $LDB_CMD"
  exit 1
fi

# Try read back
read_out=$("$LDB_CMD" --db="$ROCKSDB_PATH" get "$TEST_KEY" 2>/dev/null || true)
if [[ -z "$read_out" || "${read_out}" != *"$TEST_VALUE"* ]]; then
  echo "❌ RocksDB read test failed (got: ${read_out:-<empty>})"
  exit 1
fi

# Cleanup: attempt to delete test key (ignore errors)
"$LDB_CMD" --db="$ROCKSDB_PATH" delete "$TEST_KEY" >/dev/null 2>&1 || true

echo "✅ RocksDB connection and read/write test OK (DB: $ROCKSDB_PATH, tool: $LDB_CMD)"
exit 0
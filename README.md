# 1inch Fusion Streamer

Library for streaming 1inch Fusion events.

## CLI

Out-of-the-box CLI to stream Fusion events to either stdout or local jsonl file.

```bash
# Set environment variables (or provide them as params) 
export ONEINCH_ENDPOINT="..."
export ONEINCH_API_TOKEN="..."

# Specify network ID and output file (stream to stdout if missing)
cargo run -r -p local -- --network-id 1 --out-path events.jsonl
```
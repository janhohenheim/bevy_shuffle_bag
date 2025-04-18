#!/bin/bash
set -euo pipefail

cargo expand > expanded.rs
cp expanded.rs ../expanded.rs

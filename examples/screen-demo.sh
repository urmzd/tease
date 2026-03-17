#!/bin/bash
# Screen capture demo script for teasr showcase
cd "$(dirname "$0")/.." 2>/dev/null || true

printf '\033[1;36m╔══════════════════════════════════════╗\033[0m\n'
printf '\033[1;36m║\033[1;33m   teasr \033[0;37m— showcase capture tool   \033[1;36m║\033[0m\n'
printf '\033[1;36m╚══════════════════════════════════════╝\033[0m\n'
echo
tree --dirsfirst -C -L 2 crates/ 2>/dev/null || find crates -maxdepth 2 -type f | sort
echo
printf '\033[1;32m✓ ready\033[0m\n'
sleep 120

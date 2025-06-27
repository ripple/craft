#!/bin/bash

# Colors for output
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Stopping Craft UI servers...${NC}"

# Kill backend server
pkill -f "node server.js" 2>/dev/null

# Kill frontend servers (various possible servers)
pkill -f "http.server 8080" 2>/dev/null
pkill -f "SimpleHTTPServer 8080" 2>/dev/null
pkill -f "http-server.*8080" 2>/dev/null

echo -e "${GREEN}Craft UI servers stopped.${NC}"
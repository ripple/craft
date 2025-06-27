#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔨 Starting Craft UI...${NC}\n"

# Check if node is installed
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}Node.js is not installed. Please install Node.js first.${NC}"
    exit 1
fi

# Get the script directory (where craft project is)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Check if backend dependencies are installed
if [ ! -d "$SCRIPT_DIR/craft-ui/backend/node_modules" ]; then
    echo -e "${YELLOW}Installing backend dependencies...${NC}"
    cd "$SCRIPT_DIR/craft-ui/backend" && npm install
fi

# Start the backend server
echo -e "${GREEN}Starting backend server on http://localhost:3001${NC}"
cd "$SCRIPT_DIR/craft-ui/backend" && node server.js &
BACKEND_PID=$!

# Wait a moment for the server to start
sleep 2

# Function to open URL in default browser (cross-platform)
open_browser() {
    local url=$1
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        if command -v xdg-open &> /dev/null; then
            xdg-open "$url"
        elif command -v gnome-open &> /dev/null; then
            gnome-open "$url"
        else
            echo -e "${YELLOW}Could not detect browser. Please open manually: $url${NC}"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        open "$url"
    elif [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        # Windows
        start "$url"
    else
        echo -e "${YELLOW}Could not detect OS. Please open manually: $url${NC}"
    fi
}

# Start a simple HTTP server for the frontend
echo -e "${GREEN}Starting frontend server on http://localhost:8080${NC}"

# Change to frontend directory
cd "$SCRIPT_DIR/craft-ui/frontend"

# Try Python 3 first, then Python 2, then fallback to npx http-server
if command -v python3 &> /dev/null; then
    python3 -m http.server 8080 &
    FRONTEND_PID=$!
elif command -v python &> /dev/null; then
    python -m SimpleHTTPServer 8080 &
    FRONTEND_PID=$!
else
    # Use npx to run http-server without installing globally
    npx -y http-server -p 8080 &
    FRONTEND_PID=$!
fi

# Wait a moment for the frontend server to start
sleep 2

# Open the browser
echo -e "${GREEN}Opening Craft UI in your browser...${NC}"
open_browser "http://localhost:8080"

echo -e "\n${BLUE}Craft UI is running!${NC}"
echo -e "Frontend: ${GREEN}http://localhost:8080${NC}"
echo -e "Backend API: ${GREEN}http://localhost:3001${NC}"
echo -e "\nPress ${YELLOW}Ctrl+C${NC} to stop all servers\n"

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Shutting down Craft UI...${NC}"
    
    # Kill backend server
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null
    fi
    
    # Kill frontend server
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null
    fi
    
    # Also kill any orphaned node processes running our server
    pkill -f "node server.js" 2>/dev/null
    pkill -f "http.server 8080" 2>/dev/null
    pkill -f "SimpleHTTPServer 8080" 2>/dev/null
    pkill -f "http-server.*8080" 2>/dev/null
    
    echo -e "${GREEN}Craft UI stopped.${NC}"
    exit 0
}

# Set up trap to cleanup on Ctrl+C
trap cleanup INT TERM

# Wait for Ctrl+C
wait
# Craft UI - Smart Escrow Management Web Interface

A simple, minimal-dependency web interface for managing XRPL smart escrows using the Craft CLI tool.

## Features

1. **Code Editor** - View and edit KYC project code with syntax highlighting
2. **Build & Deploy** - Build WASM modules and deploy to WASM Devnet
3. **Transaction Manager** - Create and submit escrow/credential transactions
4. **Status Dashboard** - View deployment information and results

## Quick Start

### 1. Install Backend Dependencies

```bash
cd craft-ui/backend
npm install
```

### 2. Start the Backend Server

```bash
npm start
```

The backend will run on http://localhost:3001

### 3. Open the Frontend

Open `craft-ui/frontend/index.html` in your web browser, or serve it with a simple HTTP server:

```bash
cd craft-ui/frontend
python3 -m http.server 8080
# or
npx http-server .
```

Then navigate to http://localhost:8080

## Usage

### 1. Code Editor Tab
- View the KYC project code
- Make small edits
- Click "Save Changes" to update the code

### 2. Build & Deploy Tab
- Select build mode (Release/Debug)
- Choose optimization level
- Click "Build WASM" to compile
- Once built, click "Deploy to WASM Devnet"
- Server is pre-filled with: `wss://wasm.devnet.rippletest.net:51233`

### 3. Transactions Tab
Follow the three-step process:
1. **First Escrow Finish** - Will fail (no credential)
2. **Create KYC Credential** - Create the required credential
3. **Second Escrow Finish** - Will succeed

All forms are auto-populated with deployment data.

### 4. Status Tab
View the latest deployment information including:
- Account addresses
- Escrow sequence number
- Deployment timestamp

## Architecture

- **Frontend**: Plain HTML/CSS/JavaScript with CodeMirror for code editing
- **Backend**: Node.js/Express server that interfaces with the Craft CLI
- **Storage**: In-memory session storage + file outputs to `tx_responses.json`

## Dependencies

Minimal dependencies for simplicity:

Backend:
- express
- cors
- body-parser

Frontend:
- CodeMirror (loaded from CDN)

## Notes

- Single-user demo application (no authentication)
- Deployment data persists in memory until server restart
- Transaction results are appended to `tx_responses.json`
- Requires the Craft CLI to be built and available at `../../target/release/craft`
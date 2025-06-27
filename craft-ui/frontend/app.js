// API base URL
const API_URL = 'http://localhost:3001/api';

// Global variables
let codeEditor;
let deploymentInfo = null;
let wasmPath = null;

// Initialize CodeMirror
document.addEventListener('DOMContentLoaded', async () => {
    // Initialize code editor
    codeEditor = CodeMirror.fromTextArea(document.getElementById('codeEditor'), {
        mode: 'rust',
        theme: 'monokai',
        lineNumbers: true,
        lineWrapping: true,
        autoCloseBrackets: true,
        matchBrackets: true
    });

    // Load initial code
    await loadCode();

    // Set up tab navigation
    setupTabs();

    // Set up event listeners
    setupEventListeners();

    // Check for existing deployment info
    await checkDeploymentInfo();
});

// Tab navigation
function setupTabs() {
    const tabButtons = document.querySelectorAll('.tab-button');
    const tabContents = document.querySelectorAll('.tab-content');

    tabButtons.forEach(button => {
        button.addEventListener('click', () => {
            const tabName = button.dataset.tab;

            // Update active states
            tabButtons.forEach(btn => btn.classList.remove('active'));
            tabContents.forEach(content => content.classList.remove('active'));

            button.classList.add('active');
            document.getElementById(tabName).classList.add('active');
        });
    });
}

// Event listeners
function setupEventListeners() {
    // Code editor
    document.getElementById('saveCode').addEventListener('click', saveCode);

    // Build & Deploy
    document.getElementById('buildBtn').addEventListener('click', buildWasm);
    document.getElementById('deployBtn').addEventListener('click', deployWasm);

    // Transactions
    document.getElementById('escrow1Btn').addEventListener('click', () => finishEscrow(1));
    document.getElementById('credBtn').addEventListener('click', createCredential);
    document.getElementById('escrow2Btn').addEventListener('click', () => finishEscrow(2));
}

// Load code from backend
async function loadCode() {
    try {
        const response = await fetch(`${API_URL}/code/kyc`);
        const data = await response.json();
        codeEditor.setValue(data.code);
    } catch (error) {
        console.error('Error loading code:', error);
        showOutput('buildOutput', 'Error loading code: ' + error.message);
    }
}

// Save code
async function saveCode() {
    const button = document.getElementById('saveCode');
    button.disabled = true;
    button.textContent = 'Saving...';

    try {
        const response = await fetch(`${API_URL}/code/kyc`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ code: codeEditor.getValue() })
        });

        if (response.ok) {
            button.textContent = 'Saved!';
            setTimeout(() => {
                button.textContent = 'Save Changes';
                button.disabled = false;
            }, 2000);
        } else {
            throw new Error('Failed to save code');
        }
    } catch (error) {
        console.error('Error saving code:', error);
        alert('Error saving code: ' + error.message);
        button.textContent = 'Save Changes';
        button.disabled = false;
    }
}

// Build WASM
async function buildWasm() {
    const button = document.getElementById('buildBtn');
    const output = document.getElementById('buildOutput');
    const deployBtn = document.getElementById('deployBtn');

    button.disabled = true;
    button.textContent = 'Building...';
    deployBtn.disabled = true;
    showOutput('buildOutput', 'Building WASM module...\n');

    try {
        const buildMode = document.getElementById('buildMode').value;
        const optimization = document.getElementById('optimization').value;

        const response = await fetch(`${API_URL}/build`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ buildMode, optimization })
        });

        const data = await response.json();

        if (data.success) {
            wasmPath = data.wasmPath;
            // Build successful
            showOutput('buildOutput', data.output + '\n\nBuild successful!');
            deployBtn.disabled = false;
        } else {
            showOutput('buildOutput', 'Build failed:\n' + (data.error || data.output));
        }
    } catch (error) {
        console.error('Error building:', error);
        showOutput('buildOutput', 'Error building: ' + error.message);
    } finally {
        button.disabled = false;
        button.textContent = 'Build WASM';
    }
}

// Deploy WASM
async function deployWasm() {
    const button = document.getElementById('deployBtn');
    const server = document.getElementById('deployServer').value;

    button.disabled = true;
    button.textContent = 'Deploying...';
    showOutput('buildOutput', '\n\nDeploying to ' + server + '...\n', true);

    try {
        const response = await fetch(`${API_URL}/deploy`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ server, wasmPath })
        });

        const data = await response.json();

        if (data.success) {
            // Merge deployment info to preserve fingerprint from build
            deploymentInfo = { ...deploymentInfo, ...data.deployment };
            console.log('Deployment info:', deploymentInfo);
            showOutput('buildOutput', data.output + '\n\nDeployment successful!', true);
            updateTransactionForms();
            updateStatus();
        } else {
            showOutput('buildOutput', 'Deployment failed:\n' + (data.error || data.output), true);
        }
    } catch (error) {
        console.error('Error deploying:', error);
        showOutput('buildOutput', 'Error deploying: ' + error.message, true);
    } finally {
        button.disabled = false;
        button.textContent = 'Deploy to WASM Devnet';
    }
}

// Finish escrow
async function finishEscrow(attempt) {
    const output = document.getElementById('transactionOutput');
    const account = document.getElementById('escrow1Account').value;
    const secret = document.getElementById('escrow1Secret').value;
    const owner = document.getElementById('escrow1Owner').value;
    const sequence = document.getElementById('escrow1Sequence').value;
    const server = document.getElementById('deployServer').value;

    if (!account || !secret || !owner || !sequence) {
        alert('Please fill in all fields');
        return;
    }

    showOutput('transactionOutput', `\nAttempt ${attempt}: Submitting EscrowFinish transaction...\n`, true);

    try {
        const response = await fetch(`${API_URL}/escrow/finish`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ 
                account, 
                secret, 
                owner, 
                sequence, 
                server
            })
        });

        const data = await response.json();
        showOutput('transactionOutput', data.output || data.error, true);
    } catch (error) {
        console.error('Error finishing escrow:', error);
        showOutput('transactionOutput', 'Error: ' + error.message, true);
    }
}

// Create credential
async function createCredential() {
    const account = document.getElementById('credAccount').value;
    const secret = document.getElementById('credSecret').value;
    const subject = document.getElementById('credSubject').value;
    const server = document.getElementById('deployServer').value;

    if (!account || !secret || !subject) {
        alert('Please fill in all fields');
        return;
    }

    showOutput('transactionOutput', '\nCreating KYC credential...\n', true);

    try {
        const response = await fetch(`${API_URL}/credential/create`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ account, secret, subject, server })
        });

        const data = await response.json();
        showOutput('transactionOutput', data.output || data.error, true);
    } catch (error) {
        console.error('Error creating credential:', error);
        showOutput('transactionOutput', 'Error: ' + error.message, true);
    }
}

// Update transaction forms with deployment info
function updateTransactionForms() {
    if (!deploymentInfo) return;

    // Update escrow finish forms
    document.getElementById('escrow1Account').value = deploymentInfo.account2 || '';
    document.getElementById('escrow1Secret').value = deploymentInfo.secret2 || '';
    document.getElementById('escrow1Owner').value = deploymentInfo.account1 || '';
    document.getElementById('escrow1Sequence').value = deploymentInfo.escrowSequence || '';

    // Update credential form
    document.getElementById('credAccount').value = deploymentInfo.account2 || '';
    document.getElementById('credSecret').value = deploymentInfo.secret2 || '';
    document.getElementById('credSubject').value = deploymentInfo.account2 || '';
}

// Update status dashboard
function updateStatus() {
    const statusContent = document.getElementById('statusContent');
    
    if (!deploymentInfo) {
        statusContent.innerHTML = '<p>No deployment information available yet.</p>';
        return;
    }

    statusContent.innerHTML = `
        <h3>Latest Deployment</h3>
        <div class="status-item">
            <strong>Timestamp:</strong> ${deploymentInfo.timestamp || 'N/A'}
        </div>
        <div class="status-item">
            <strong>Server:</strong> ${deploymentInfo.server || 'N/A'}
        </div>
        <div class="status-item">
            <strong>Account 1 (Origin):</strong> ${deploymentInfo.account1 || 'N/A'}
        </div>
        <div class="status-item">
            <strong>Account 2 (Destination):</strong> ${deploymentInfo.account2 || 'N/A'}
        </div>
        <div class="status-item">
            <strong>Escrow Sequence:</strong> ${deploymentInfo.escrowSequence || 'N/A'}
        </div>
    `;
}

// Check for existing deployment info
async function checkDeploymentInfo() {
    try {
        const response = await fetch(`${API_URL}/deployment-info`);
        if (response.ok) {
            deploymentInfo = await response.json();
            updateTransactionForms();
            updateStatus();
        }
    } catch (error) {
        console.error('Error checking deployment info:', error);
    }
}

// Show output in specified element
function showOutput(elementId, text, append = false) {
    const element = document.getElementById(elementId);
    if (append) {
        element.textContent += text;
    } else {
        element.textContent = text;
    }
    element.scrollTop = element.scrollHeight;
}
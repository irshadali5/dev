#!/usr/bin/env bash
# ==============================================================================
# One-Click Deployment to Server for irshadali.org
# Reads credentials and host from .env
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${SCRIPT_DIR}/.env"

if [ -f "$ENV_FILE" ]; then
    # Load variables from .env
    eval "$(grep -v '^#' "$ENV_FILE" | grep -E '^(IP_ADDRESSE|KEY|USER|DOMAIN)=' | sed 's/ *= */=/g')"
fi

DOMAIN="${DOMAIN:-irshadali.org}"
SERVER_IP="${IP_ADDRESSE:-20.198.120.162}"
SSH_KEY="${KEY:-$HOME/secure/queos-server_key.pem}"
SSH_USER="${USER:-irshad}"
REMOTE_DIR="~/deploy_${DOMAIN//./_}"

# Expand tilde in key path if needed
SSH_KEY="${SSH_KEY/#\~/$HOME}"

echo "=================================================="
echo " Deploying ${DOMAIN} to Remote Server"
echo " Host: ${SSH_USER}@${SERVER_IP}"
echo " Key:  ${SSH_KEY}"
echo "=================================================="

if [ ! -f "$SSH_KEY" ]; then
    echo "[-] Error: SSH Key not found at ${SSH_KEY}"
    exit 1
fi

SSH_OPTS="-o StrictHostKeyChecking=no -o ConnectTimeout=10 -i ${SSH_KEY}"

echo "[1/4] Verifying SSH connectivity..."
if ! ssh ${SSH_OPTS} "${SSH_USER}@${SERVER_IP}" "echo '[+] SSH Connection established successfully'" ; then
    echo "[-] Could not connect to ${SSH_USER}@${SERVER_IP}."
    echo "    Please verify that the server is powered on, port 22 is open in security groups, and IP is correct."
    exit 1
fi

echo "[2/4] Packaging and uploading build bundle..."
ssh ${SSH_OPTS} "${SSH_USER}@${SERVER_IP}" "mkdir -p ${REMOTE_DIR}"
tar -czf - -C "${SCRIPT_DIR}/deploy" . | ssh ${SSH_OPTS} "${SSH_USER}@${SERVER_IP}" "tar -xzf - -C ${REMOTE_DIR}"

echo "[3/4] Running remote server setup..."
ssh ${SSH_OPTS} -t "${SSH_USER}@${SERVER_IP}" "cd ${REMOTE_DIR} && chmod +x setup.sh && sudo DOMAIN='${DOMAIN}' ./setup.sh"

echo "[4/4] Verifying deployment..."
ssh ${SSH_OPTS} "${SSH_USER}@${SERVER_IP}" "curl -sI http://127.0.0.1:3000/ || true"

echo "=================================================="
echo "🎉 Deployment successfully finished!"
echo "=================================================="

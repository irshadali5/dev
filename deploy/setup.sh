#!/usr/bin/env bash
set -euo pipefail

DOMAIN="${DOMAIN:-irshadali.org}"
TARGET_DIR="${TARGET_DIR:-/var/www/${DOMAIN}}"
CURRENT_USER="$(whoami)"

echo "=================================================="
echo " Setting up ${DOMAIN} on $(hostname)"
echo " Running as user: ${CURRENT_USER}"
echo "=================================================="

# 1. Create target directory
echo "[1/6] Preparing application directory at ${TARGET_DIR}..."
sudo mkdir -p "${TARGET_DIR}/data" "${TARGET_DIR}/content" "${TARGET_DIR}/site" "${TARGET_DIR}/data/pingap"
sudo chown -R "${CURRENT_USER}:${CURRENT_USER}" "${TARGET_DIR}"

# 2. Sync application files
echo "[2/6] Copying server binary and assets..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp "${SCRIPT_DIR}/server" "${TARGET_DIR}/server"
chmod +x "${TARGET_DIR}/server"

cp -r "${SCRIPT_DIR}/site/"* "${TARGET_DIR}/site/"
cp -r "${SCRIPT_DIR}/content/"* "${TARGET_DIR}/content/"
cp -r "${SCRIPT_DIR}/data/"* "${TARGET_DIR}/data/" || true
[ -f "${SCRIPT_DIR}/dev_site.db" ] && cp "${SCRIPT_DIR}/dev_site.db" "${TARGET_DIR}/" || true
cp "${SCRIPT_DIR}/pingap.toml" "${TARGET_DIR}/pingap.toml"

# Update user and domain in service/proxy configs if customized
sed -i "s/User=irshad/User=${CURRENT_USER}/g" "${SCRIPT_DIR}/dev-site.service"
if [ "${DOMAIN}" != "irshadali.org" ]; then
    sed -i "s/irshadali.org/${DOMAIN}/g" "${SCRIPT_DIR}/dev-site.service" "${SCRIPT_DIR}/pingap.service" "${TARGET_DIR}/pingap.toml"
fi

# 3. Ensure Pingap reverse proxy binary is installed
echo "[3/6] Installing / Verifying Pingap reverse proxy..."
if ! command -v pingap &> /dev/null; then
    echo "Downloading Pingap release..."
    PINGAP_URL="https://github.com/vicanso/pingap/releases/latest/download/pingap-x86_64-unknown-linux-musl.tar.gz"
    curl -fsSL "$PINGAP_URL" -o /tmp/pingap.tar.gz
    sudo tar -xzf /tmp/pingap.tar.gz -C /usr/local/bin/ pingap
    sudo chmod +x /usr/local/bin/pingap
    rm -f /tmp/pingap.tar.gz
    echo "Pingap installed at /usr/local/bin/pingap"
fi

# 4. Configure firewall (UFW / iptables)
echo "[4/6] Configuring firewall ports (80, 443)..."
if command -v ufw &> /dev/null; then
    sudo ufw allow 80/tcp || true
    sudo ufw allow 443/tcp || true
fi

# 5. Install and enable systemd services
echo "[5/6] Registering systemd services..."
sudo cp "${SCRIPT_DIR}/dev-site.service" /etc/systemd/system/dev-site.service
sudo cp "${SCRIPT_DIR}/pingap.service" /etc/systemd/system/pingap.service

sudo systemctl daemon-reload
sudo systemctl enable dev-site.service
sudo systemctl enable pingap.service

sudo systemctl restart dev-site.service
sudo systemctl restart pingap.service

# 6. Verification
echo "[6/6] Verifying service statuses..."
sleep 2

sudo systemctl status dev-site.service --no-pager || true
sudo systemctl status pingap.service --no-pager || true

echo "=================================================="
echo "🎉 Setup complete! ${DOMAIN} is live and serving."
echo "=================================================="

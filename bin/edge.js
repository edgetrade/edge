#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { existsSync, readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

function getVersion() {
  try {
    const pkg = JSON.parse(readFileSync(join(__dirname, '..', 'package.json'), 'utf8'));
    return pkg.version;
  } catch {
    return '0.1.0';
  }
}

function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  const version = getVersion();

  let binaryName;

  if (platform === 'win32') {
    binaryName = `edge-v${version}-x86_64-pc-windows-gnu.exe`;
  } else if (platform === 'darwin') {
    binaryName = arch === 'arm64' ? `edge-v${version}-aarch64-apple-darwin` : `edge-v${version}-x86_64-apple-darwin`;
  } else if (platform === 'linux') {
    binaryName =
      arch === 'arm64' ? `edge-v${version}-aarch64-unknown-linux-musl` : `edge-v${version}-x86_64-unknown-linux-musl`;
  } else {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  const binaryPath = join(__dirname, '..', 'binaries', binaryName);

  if (!existsSync(binaryPath)) {
    console.error(`Binary not found at ${binaryPath}`);
    console.error('Please run: npm install @edgedottrade/mcp');
    process.exit(1);
  }

  return binaryPath;
}

const binaryPath = getBinaryPath();
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env,
});

child.on('exit', (code) => {
  process.exit(code || 0);
});

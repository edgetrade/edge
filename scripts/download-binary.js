#!/usr/bin/env node
import { chmodSync, createWriteStream, mkdirSync } from 'node:fs';
import { get } from 'node:https';
import { dirname, join } from 'node:path';
import { pipeline } from 'node:stream/promises';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const binariesDir = join(__dirname, '..', 'binaries');

function getBinaryInfo() {
  const platform = process.platform;
  const arch = process.arch;
  const version = process.env.npm_package_version || '0.1.0';

  let target;
  let filename;

  if (platform === 'win32') {
    target = 'x86_64-pc-windows-gnu';
    filename = `edge-v${version}-${target}.exe`;
  } else if (platform === 'darwin') {
    target = arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    filename = `edge-v${version}-${target}`;
  } else if (platform === 'linux') {
    target = arch === 'arm64' ? 'aarch64-unknown-linux-musl' : 'x86_64-unknown-linux-musl';
    filename = `edge-v${version}-${target}`;
  } else {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  return { target, filename };
}

async function downloadBinary() {
  const { filename } = getBinaryInfo();
  const version = process.env.npm_package_version || '0.1.0';
  const url = `https://github.com/edgetrade/mcp/releases/download/v${version}/${filename}`;

  console.log(`Downloading edge binary for ${process.platform}-${process.arch}...`);
  console.log(`URL: ${url}`);

  mkdirSync(binariesDir, { recursive: true });

  const outputPath = join(binariesDir, filename);

  return new Promise((resolve, reject) => {
    get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        get(response.headers.location, (redirectResponse) => {
          if (redirectResponse.statusCode !== 200) {
            reject(new Error(`Download failed with status ${redirectResponse.statusCode}`));
            return;
          }

          const fileStream = createWriteStream(outputPath);
          pipeline(redirectResponse, fileStream)
            .then(() => {
              // Make executable on Unix
              if (process.platform !== 'win32') {
                chmodSync(outputPath, 0o755);
              }
              console.log('✓ Binary downloaded successfully');
              resolve();
            })
            .catch(reject);
        }).on('error', reject);
      } else if (response.statusCode === 200) {
        const fileStream = createWriteStream(outputPath);
        pipeline(response, fileStream)
          .then(() => {
            if (process.platform !== 'win32') {
              chmodSync(outputPath, 0o755);
            }
            console.log('✓ Binary downloaded successfully');
            resolve();
          })
          .catch(reject);
      } else {
        reject(new Error(`Download failed with status ${response.statusCode}`));
      }
    }).on('error', reject);
  });
}

downloadBinary().catch((error) => {
  console.error('Failed to download edge binary:', error.message);
  console.error('\nYou can manually install using:');
  console.error('  cargo install edge-trade');
  process.exit(1);
});

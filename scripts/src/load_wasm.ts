import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { pipeline } from 'stream';
import { promisify } from 'util';

const directoryPath = path.join(__dirname, '../../artifacts');
const pipelineAsync = promisify(pipeline);

function getWasmFilesPath(): string[] {
  console.log(directoryPath);

  try {
    const files = fs.readdirSync(directoryPath);
    return files.filter((file) => file.endsWith('.wasm')).map((file) => path.join(directoryPath, file));
  } catch (err) {
    console.error("Cannot find wasm folder. Are you sure you compiled the wasm projects?");
    process.exit(1);
  }
}

async function generateSha256(filePath: string): Promise<string> {
  const readStream = fs.createReadStream(filePath);
  const hash = crypto.createHash('sha256');

  readStream.on('data', chunk => hash.update(chunk));

  await pipelineAsync(readStream, hash);
  return hash.digest('hex');
}

export async function loadWasmFileDigest() {
  const wasmFiles = getWasmFilesPath();
  const wasmFileDigest: { [key: string]: string } = {};

  wasmFiles.forEach((file) => {
    const wasm = fs.readFileSync(file);
    const digest = wasm.toString('base64');
    wasmFileDigest[file] = digest;
  });

  await Promise.all(wasmFiles.map(async (file) => {
    const digest = await generateSha256(file);
    const fileName = path.basename(file);
    wasmFileDigest[file] = digest;
  }));

  return wasmFileDigest;
}

export function getWasmPath(contractName: string): string {
  return path.join(directoryPath, `${contractName}.wasm`);
}

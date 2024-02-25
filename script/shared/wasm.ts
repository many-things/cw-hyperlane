import fs from "fs";
import path from "path";
import { defaultArtifactPath } from "./constants";
import { generateSha256 } from "./utils";

function getWasmFilesPath(
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  }
): string[] {
  try {
    const files = fs.readdirSync(artifactPath);
    return files
      .filter((file) => file.endsWith(".wasm"))
      .map((file) => path.join(artifactPath, file));
  } catch (err) {
    console.error(
      "Cannot find wasm folder. Are you sure you compiled the wasm projects?"
    );
    process.exit(1);
  }
}

export async function loadWasmFileDigest(
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  }
): Promise<Record<string, string>> {
  return Object.fromEntries(
    await Promise.all(getWasmFilesPath({ artifactPath }).map(generateSha256))
  );
}

export function getWasmPath(
  contractName: string,
  { artifactPath }: { artifactPath: string } = {
    artifactPath: defaultArtifactPath,
  }
): string {
  return path.join(artifactPath, `${contractName}.wasm`);
}

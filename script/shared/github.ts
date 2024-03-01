import * as fs from "fs";
import path from "path";
import decompress from "decompress";

import { downloadFile } from "./utils";

const RELEASE_API_URL =
  "https://api.github.com/repos/many-things/cw-hyperlane/releases";

const RELEASE_ARTIFACT_URL = (tag: string) =>
  `https://github.com/many-things/cw-hyperlane/releases/download/${tag}/cw-hyperlane-${tag}.zip`;

export const MIN_RELEASE_VERSION = "v0.0.6-rc8";

interface ReleaseApiResp {
  tag_name: string;
}

export const getReleases = async (): Promise<Record<string, string>> => {
  const apiResp = await fetch(RELEASE_API_URL);
  const releases = (await apiResp.json()) as ReleaseApiResp[];

  return Object.fromEntries(
    releases
      .map((v) => v.tag_name)
      .filter((v) => v >= MIN_RELEASE_VERSION)
      .map((v) => [v, RELEASE_ARTIFACT_URL(v)])
  );
};

export const downloadReleases = async (
  codeUrl: string,
  outPath: string
): Promise<string> => {
  const codeFile = path.join(outPath, "codes.zip");
  const unzipPath = path.join(outPath, "codes");

  if (fs.existsSync(codeFile)) fs.rmSync(codeFile);
  if (fs.existsSync(unzipPath)) fs.rmSync(unzipPath, { recursive: true });

  await downloadFile(codeUrl, codeFile);
  await decompress(codeFile, unzipPath);

  return unzipPath;
};

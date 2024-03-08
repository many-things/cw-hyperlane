import decompress from 'decompress';
import * as fs from 'fs';
import path from 'path';

import {
  RELEASE_API_URL,
  RELEASE_ARTIFACT_URL,
  REMOTE_MIN_VERSION,
} from './constants';
import { downloadFile } from './utils';

interface ReleaseApiResp {
  tag_name: string;
}

export const getReleases = async (): Promise<Record<string, string>> => {
  const apiResp = await fetch(RELEASE_API_URL);
  const releases = (await apiResp.json()) as ReleaseApiResp[];

  return Object.fromEntries(
    releases
      .map((v) => v.tag_name)
      .filter((v) => v >= REMOTE_MIN_VERSION)
      .map((v) => [v, RELEASE_ARTIFACT_URL(v)]),
  );
};

export const downloadReleases = async (
  codeUrl: string,
  outPath: string,
): Promise<string> => {
  const codeFile = path.join(outPath, 'codes.zip');
  const unzipPath = path.join(outPath, 'codes');

  if (fs.existsSync(codeFile)) fs.rmSync(codeFile);
  if (fs.existsSync(unzipPath)) fs.rmSync(unzipPath, { recursive: true });

  await downloadFile(codeUrl, codeFile);
  await decompress(codeFile, unzipPath);

  return unzipPath;
};

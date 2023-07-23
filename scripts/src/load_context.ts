import fs from 'fs';
import path from 'path';
import { Context } from './types';


const directoryPath = path.join(__dirname, '../context');

export function loadContext(network: string): Context  {
  try {
    const fileName = path.join(directoryPath, `${network}.json`);
    const result = fs.readFileSync(fileName);

    return JSON.parse(result.toString()) as Context;
  }
  catch (err) {}

  return { contracts: {}};
}

export function saveContext(network: string, context: Context) {
  const fileName = path.join(directoryPath, `${network}.json`);
  fs.writeFileSync(fileName, JSON.stringify(context, null, 2));
}

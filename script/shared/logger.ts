export class Logger {
  constructor(public name: string) {}

  log(...args: unknown[]) {
    console.log(`[${this.name}]`, ...args);
  }

  debug = (...args: unknown[]) =>
    console.log('DEBUG]'.grey, `[${this.name}]`, ...args);

  info = (...args: unknown[]) =>
    console.log(' INFO]'.cyan, `[${this.name}]`, ...args);

  error = (...args: unknown[]) =>
    console.error('ERROR]'.red, `[${this.name}]`, ...args);
}

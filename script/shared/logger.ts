export class Logger {
  constructor(public name: string) {}

  log(...args: any[]) {
    console.log(`[${this.name}]`, ...args);
  }

  debug = (...args: any[]) =>
    console.log("DEBUG]".grey, `[${this.name}]`, ...args);

  info = (...args: any[]) =>
    console.log(" INFO]".cyan, `[${this.name}]`, ...args);

  error = (...args: any[]) =>
    console.error("ERROR]".red, `[${this.name}]`, ...args);
}

export interface PtySessionOptions {
  command: string;
  args?: string[];
  cols?: number;
  rows?: number;
  env?: Record<string, string>;
}

export class PtyTestSession {
  private outputBuffer = "";
  // biome-ignore lint/suspicious/noExplicitAny: node-pty instance type
  private ptyProcess: any = null;

  constructor(private readonly options: PtySessionOptions) {}

  public async start(): Promise<void> {
    try {
      // Dynamic import to allow optional native node-pty loading
      const pty = await import("node-pty");
      const sanitizedEnv: Record<string, string> = {};
      for (const [k, v] of Object.entries(process.env)) {
        if (v !== undefined) sanitizedEnv[k] = v;
      }
      if (this.options.env) {
        Object.assign(sanitizedEnv, this.options.env);
      }

      this.ptyProcess = pty.spawn(this.options.command, this.options.args || [], {
        name: "xterm-256color",
        cols: this.options.cols || 80,
        rows: this.options.rows || 24,
        cwd: process.cwd(),
        env: sanitizedEnv,
      });

      this.ptyProcess.onData((data: string) => {
        this.outputBuffer += data;
      });
    } catch {
      console.warn("node-pty not found or native build unavailable — running in mock PTY mode.");
    }
  }

  public write(data: string): void {
    if (this.ptyProcess) {
      this.ptyProcess.write(data);
    }
  }

  public resize(cols: number, rows: number): void {
    if (this.ptyProcess) {
      this.ptyProcess.resize(cols, rows);
    }
  }

  public getBuffer(): string {
    return this.outputBuffer;
  }

  public async waitForOutput(pattern: string | RegExp, timeoutMs = 5000): Promise<void> {
    const startTime = Date.now();
    while (Date.now() - startTime < timeoutMs) {
      const buf = this.getBuffer();
      const matched = typeof pattern === "string" ? buf.includes(pattern) : pattern.test(buf);
      if (matched) return;
      await new Promise((r) => setTimeout(r, 50));
    }
    throw new Error(
      `Timeout waiting for PTY output matching: ${pattern}\nBuffer:\n${this.getBuffer()}`,
    );
  }

  public async stop(): Promise<number> {
    if (!this.ptyProcess) return 0;
    return new Promise((resolve) => {
      this.ptyProcess.onExit((e: { exitCode: number }) => resolve(e.exitCode));
      this.ptyProcess.kill();
    });
  }
}

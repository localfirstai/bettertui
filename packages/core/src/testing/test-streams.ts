import { Readable, Writable } from "node:stream";

export class TestWriteStream extends Writable {
  public readonly isTTY = true;
  public columns: number;
  public rows: number;
  public buffer: Buffer = Buffer.alloc(0);

  constructor(columns = 80, rows = 24) {
    super();
    this.columns = columns;
    this.rows = rows;
  }

  override _write(
    chunk: Buffer,
    _encoding: BufferEncoding,
    callback: (error?: Error | null) => void,
  ): void {
    this.buffer = Buffer.concat([this.buffer, chunk]);
    callback();
  }

  getColorDepth(): number {
    return 24;
  }

  getOutput(): string {
    return this.buffer.toString("utf8");
  }

  clear(): void {
    this.buffer = Buffer.alloc(0);
  }
}

export type TestStdout = TestWriteStream & NodeJS.WriteStream;

export class TestReadStream extends Readable {
  public readonly isTTY = true;

  constructor() {
    super({ read() {} });
  }

  emitData(data: string | Buffer): void {
    this.push(Buffer.from(data));
  }
}

export type TestStdin = TestReadStream & NodeJS.ReadStream;

export function createTestStdin(): TestStdin {
  return new TestReadStream() as TestStdin;
}

export function createTestStdout(columns = 80, rows = 24): TestStdout {
  return new TestWriteStream(columns, rows) as TestStdout;
}

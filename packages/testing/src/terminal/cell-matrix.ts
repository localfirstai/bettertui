export interface CellAttributes {
  char: string;
  fg?: string;
  bg?: string;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
}

export class CellMatrix {
  public readonly width: number;
  public readonly height: number;
  private matrix: CellAttributes[][];

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.matrix = this.createEmptyMatrix(width, height);
  }

  private createEmptyMatrix(w: number, h: number): CellAttributes[][] {
    const grid: CellAttributes[][] = [];
    for (let y = 0; y < h; y++) {
      const row: CellAttributes[] = [];
      for (let x = 0; x < w; x++) {
        row.push({ char: " " });
      }
      grid.push(row);
    }
    return grid;
  }

  public resize(width: number, height: number): void {
    const newMatrix = this.createEmptyMatrix(width, height);
    for (let y = 0; y < Math.min(this.height, height); y++) {
      for (let x = 0; x < Math.min(this.width, width); x++) {
        newMatrix[y][x] = { ...this.matrix[y][x] };
      }
    }
    (this as { width: number }).width = width;
    (this as { height: number }).height = height;
    this.matrix = newMatrix;
  }

  public getCell(x: number, y: number): CellAttributes | undefined {
    if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
      return undefined;
    }
    return this.matrix[y][x];
  }

  public setCell(x: number, y: number, cell: Partial<CellAttributes>): void {
    if (x < 0 || x >= this.width || y < 0 || y >= this.height) {
      return;
    }
    this.matrix[y][x] = {
      ...this.matrix[y][x],
      ...cell,
    };
  }

  public writeString(
    x: number,
    y: number,
    text: string,
    style?: Omit<CellAttributes, "char">,
  ): void {
    if (y < 0 || y >= this.height) return;
    let currX = x;
    for (const char of text) {
      if (currX >= this.width) break;
      if (currX >= 0) {
        this.setCell(currX, y, { char, ...style });
      }
      currX++;
    }
  }

  public clear(): void {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        this.matrix[y][x] = { char: " " };
      }
    }
  }

  public renderTextFrame(): string {
    return this.matrix.map((row) => row.map((c) => c.char || " ").join("")).join("\n");
  }

  public renderAnsiFrame(): string {
    return this.matrix
      .map((row) => {
        let line = "";
        for (const cell of row) {
          let prefix = "";
          let suffix = "";
          if (cell.bold) {
            prefix += "\x1b[1m";
            suffix += "\x1b[22m";
          }
          if (cell.underline) {
            prefix += "\x1b[4m";
            suffix += "\x1b[24m";
          }
          line += prefix + (cell.char || " ") + suffix;
        }
        return line;
      })
      .join("\n");
  }
}

import { assert } from "./assert";

export class Segment {
  constructor(
    readonly id: string,
    readonly iteration: number,
  ) {
    assert(Number.isInteger(iteration), "n must be an integer");
    assert(iteration >= 0), "n must not be negative";
  }

  static parse(text: string): Segment {
    const match = text.match(/^([^/]+):([0-9]{1,10})$/);
    assert(match !== null, "invalid segment string");
    return new Segment(match[1]!, Number.parseInt(match[2]!));
  }

  fmt(): string {
    return `${this.id}:${this.iteration}`;
  }

  eq(other: Segment): boolean {
    return this.fmt() === other.fmt();
  }
}

export class Path {
  readonly segments: readonly Segment[];

  constructor(segments: Segment[] = []) {
    this.segments = segments.slice();
  }

  static parse(text: string): Path {
    if (text === "") return new Path();
    return new Path(text.split("/").map((it) => Segment.parse(it)));
  }

  fmt(): string {
    return this.segments.map((it) => it.fmt()).join("/");
  }

  eq(other: Path): boolean {
    return this.fmt() === other.fmt();
  }

  slice(start?: number, end?: number): Path {
    return new Path(this.segments.slice(start, end));
  }

  concat(...other: (Path | Segment)[]): Path {
    const result = this.segments.slice();
    for (const part of other) {
      if (part instanceof Segment) result.push(part);
      else result.push(...part.segments);
    }
    return new Path(result);
  }

  parent(): Path | undefined {
    if (this.segments.length === 0) return undefined;
    return this.slice(0, -1);
  }

  /**
   * All ancestors of this path (including the path itself), ordered by
   * decreasing length.
   */
  ancestors(): Path[] {
    const result = [];
    for (let i = this.segments.length; i >= 0; i--) {
      result.push(this.slice(0, i));
    }
    return result;
  }

  isPrefixOf(path: Path): boolean {
    if (path.segments.length < this.segments.length) return false;
    return path.slice(0, this.segments.length).eq(this);
  }
}

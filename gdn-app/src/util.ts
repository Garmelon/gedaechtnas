function pathString(path: number[]): string {
  return path.join("/");
}

function pathParse(path: string): number[] {
  if (path === "") return [];
  return path.split("/").map((it) => Number.parseInt(it));
}

export function pathAppend(path: string, segment: number): string {
  return pathString(pathParse(path).concat(segment));
}

export function pathAncestors(path: string): string[] {
  const parsedPath = pathParse(path);
  const result = [];
  for (let i = parsedPath.length; i >= 0; i--) {
    result.push(pathString(parsedPath.slice(0, i)));
  }
  return result;
}

export function pathLiesOn(p1: string, p2: string): boolean {
  return pathAncestors(p2).indexOf(p1) >= 0;
}

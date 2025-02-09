function pathString(keys: string[]): string {
  return keys.join("/");
}

function pathParse(path: string): string[] {
  if (path === "") return [];
  return path.split("/");
}

export function pathSlice(path: string, start: number, end?: number): string {
  return pathString(pathParse(path).slice(start, end));
}

export function pathAppend(path: string, key: string): string {
  return pathString(pathParse(path).concat(key));
}

export function pathAncestor(path: string): string {
  return pathSlice(path, 0, -1);
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

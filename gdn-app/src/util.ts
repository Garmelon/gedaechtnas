function pathString(keys: string[]): string {
  return keys.join("/");
}

function pathParse(path: string): string[] {
  if (path === "") return [];
  return path.split("/");
}

export function pathAppend(path: string, key: string): string {
  return pathString(pathParse(path).concat(key));
}

export function pathAncestor(path: string): string {
  return pathString(pathParse(path).slice(0, -1));
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

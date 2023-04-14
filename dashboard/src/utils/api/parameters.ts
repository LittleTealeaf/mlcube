import { ApiType } from "@/types/api";

export function getParameters<T extends ApiType>(
  request: Request
): T["params"] {
  const { searchParams } = new URL(request.url);
  const parameters = searchParams.entries();
  return Object.fromEntries(parameters) as T["params"];
}

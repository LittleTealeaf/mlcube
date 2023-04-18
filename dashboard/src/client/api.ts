import { ApiType } from "@/types/api";
import useSWR, { SWRConfiguration } from "swr";

export type UseApiParameters<T extends ApiType, O = T["response"]> = {
  url: T["url"];
  params: T["params"];
  init?: RequestInit;
  config?: SWRConfiguration;
  postProcess?: (data: T["response"]) => O;
};

export function useApi<Type extends ApiType, Response = Type["response"]>({
  url,
  params,
  init,
  config,
  postProcess,
}: UseApiParameters<Type, Response>) {
  const parameters = Object.entries(params || {}).map(
    ([key, value]) => `${key}=${String(value)}`
  );
  const fetch_url = url.concat("?", parameters.join("&"));

  return useSWR(
    fetch_url,
    (url) =>
      fetch(url, params)
        .then((response) => response.json() as Type["response"])
        .then(postProcess || ((data) => data)),
    config
  );
}

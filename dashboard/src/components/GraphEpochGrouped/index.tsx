"use client";
import { useApi } from "@/client/api";
import { GraphEpochGrouped } from "@/types/api";

export type Props = {
  params: GraphEpochGrouped["params"];
};

export default function GraphEpochGrouped({ params }: Props) {
  const { data } = useApi<GraphEpochGrouped, GraphEpochGrouped["response"][]>({
    url: "/api/graph/epoch/grouped",
    params,
    postProcess: (data) => [data],
    config: {
      refreshInterval: 10000,
    },
  });

  return <></>;
}

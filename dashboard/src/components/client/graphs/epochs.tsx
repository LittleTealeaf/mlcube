"use client";

import { ResponsiveLine } from "@nivo/line";
import { Paper } from "@mui/material";
import { ApiGraphEpoch, GraphResponse } from "@/types/api";
import { useApi } from "@/utils/app/api";
import { WithSx } from "@/types/props";

// TODO: Fetch more points, and only display as much as the screen can show?

// TODO: Remove the option for "Reward"? since that's a less useful statistic?

// TODO: Add in a "scale" to allow the user to modify the view window. Maybe do this with better control over data points (increasing to 500 points and reducing client side? maybe even 1000?)

export type EpochGraphProps = {
  params: ApiGraphEpoch["params"];
} & WithSx;

export function EpochGraph({ params, sx }: EpochGraphProps) {
  const { data } = useApi<ApiGraphEpoch, GraphResponse[]>({
    url: "/api/graph/epoch",
    params,
    postProcess: (data) => [data],
    config: {
      refreshInterval: 60000,
    },
  });

  return (
    <Paper sx={sx}>
      <ResponsiveLine
        data={data || []}
        useMesh={true}
        xScale={{ type: "linear", min: "auto", max: "auto" }}
        yScale={{ type: "linear", min: "auto", max: "auto" }}
        margin={{ top: 30, left: 60, right: 30, bottom: 60 }}
        axisBottom={{
          legend: "Epoch",
          legendPosition: "middle",
          legendOffset: 50,
          tickRotation: 45,
        }}
        axisLeft={{
          legend: `Average ${params.select[0]
            .toUpperCase()
            .concat(params.select.substring(1))}`,
          legendPosition: "middle",
          legendOffset: -52,
          tickRotation: 45,
        }}
      />
    </Paper>
  );
}

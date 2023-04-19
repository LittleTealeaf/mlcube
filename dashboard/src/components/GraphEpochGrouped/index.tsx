"use client";
import { useApi } from "@/client/api";
import { GraphEpochGrouped } from "@/types/api";
import { WithSx } from "@/types/props";
import { Paper } from "@mui/material";
import { ResponsiveLine } from "@nivo/line";

export type Props = {
	params: GraphEpochGrouped["params"];
} & WithSx;

export default function GraphEpochGrouped({ params, sx }: Props) {
	const { data } = useApi<GraphEpochGrouped, GraphEpochGrouped["response"][]>({
		url: "/api/graph/epoch/grouped",
		params,
		postProcess: (data) => [data],
		config: {
			refreshInterval: 10000,
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
					legend: "Average Loss",
					legendPosition: "middle",
					legendOffset: -52,
					tickRotation: 45,
				}}
			/>
		</Paper>
	);
}

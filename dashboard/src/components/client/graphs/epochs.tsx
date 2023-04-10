'use client'

import { GraphEpochParams, GraphEpochResult } from '@/types/apitypes'
import { jsonResponse, requireStatus, useApi } from '@/utils/app/apiConsumer';
import { ResponsiveLine } from '@nivo/line';
import { Paper, SxProps, Theme } from '@mui/material';


// TODO: Fetch more points, and only display as much as the screen can show?

export function EpochGraph({ sx, ...params }: GraphEpochParams & { sx?: SxProps<Theme> }) {

	const { data } = useApi({
		url: '/api/graphs/epochs',
		params,
		process: (response) => response.then(requireStatus(200)).then(jsonResponse<GraphEpochResult>)
	})


	return (
		<Paper sx={sx}>
			<ResponsiveLine
				data={data ? [data] : []}
				useMesh={true}
				xScale={{ type: 'linear', min: 'auto', max: 'auto' }}
				yScale={{ type: 'linear', min: 'auto', max: 'auto' }}
				margin={{ top: 30, left: 60, right: 30, bottom: 60 }}
				axisBottom={{
					legend: 'Epoch',
					legendPosition: 'middle',
					legendOffset: 50,
					tickRotation: 45
				}}
				axisLeft={{
					legend: `Average ${params.select[0].toUpperCase().concat(params.select.substring(1))}`,
					legendPosition: 'middle',
					legendOffset: -52,
					tickRotation: 45
				}}
			/>
		</Paper>
	)
}

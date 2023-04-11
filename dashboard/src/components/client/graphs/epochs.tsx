'use client'

import { ResponsiveLine } from '@nivo/line';
import { Paper } from '@mui/material';
import { ApiGraphEpoch, GraphResponse } from '@/types/api';
import { useApi } from '@/utils/app/api';
import { WithSx } from '@/types/props';


// TODO: Fetch more points, and only display as much as the screen can show?

export function EpochGraph({ sx, ...params }: ApiGraphEpoch['params'] & WithSx) {

	const { data } = useApi<ApiGraphEpoch, GraphResponse[]>({
		'url': '/api/graph/epoch',
		params,
		postProcess: (data) => [data]
	});


	return (
		<Paper sx={sx}>
			<ResponsiveLine
				data={data || []}
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

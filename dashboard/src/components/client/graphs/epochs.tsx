'use client'

import useSWR from 'swr';
import { GraphEpochResult } from '@/types/apitypes'
import { getApi, jsonResponse, requireStatus } from '@/utils/app/apiConsumer';
import { ResponsiveLine } from '@nivo/line';


export type EpochGraphParams = {
	ModelId: number[],
	type: 'reward' | 'loss'
}


export function EpochGraph({ ModelId, type }: EpochGraphParams) {

	const { data: fetch_data, mutate } = useSWR<GraphEpochResult[]>(
		`/api/graphs/epochs/${type}`,
		() => Promise.all(
			ModelId.map(
				(id) => getApi(`/api/graphs/epochs/${type}`, { ModelId: id })
					.then(requireStatus(200))
					.then(jsonResponse<GraphEpochResult>))
		)
	);

	const data = fetch_data || [];

	return (
		<div style={{ height: '50vh' }}>
			<ResponsiveLine
				data={data}
				useMesh={true}
				xScale={{ type: 'linear', min: 'auto', max: 'auto' }}
				yScale={{ type: 'linear', min: 'auto', max: 'auto' }}
				margin={{ top: 30, left: 60, right: 30, bottom: 60 }}
				axisBottom={{ legend: 'Epoch', legendPosition: 'middle', legendOffset: 50, tickRotation: 45 }}
				axisLeft={{ legend: `Average ${type[0].toUpperCase().concat(type.substring(1))}`, legendPosition: 'middle', legendOffset: -52 }}
			/>
		</div>
	)
}

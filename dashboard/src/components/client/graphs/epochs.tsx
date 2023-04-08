'use client'

import useSWR from 'swr';
import { GraphEpochResult } from '@/types/apitypes'
import { getApi, jsonResponse, requireStatus } from '@/utils/app/apiConsumer';
import { ResponsiveLine } from '@nivo/line';


export type EpochGraphParams = {
	ModelId: number[]
}


export function LossGraph({ ModelId }: EpochGraphParams) {

	const { data: fetch_data, mutate } = useSWR<GraphEpochResult[]>(
		'/api/graphs/epochs/loss',
		() => Promise.all(
			ModelId.map(
				(id) => getApi('/api/graphs/epochs/loss', { ModelId: id })
					.then(requireStatus(200))
					.then(jsonResponse<GraphEpochResult>))
		)
	);

	const data = fetch_data || [];

	return (
		<div style={{height: '50vh'}}>
			<ResponsiveLine
				data={data}
				useMesh={true}
				xScale={{type: 'linear', min: 0, max: 'auto'}}
				yScale={{type: 'linear', min: 'auto', max: 'auto'}}
				margin={{top: 30, left: 60, right: 30, bottom: 30}}
			/>
		</div>
	)
}

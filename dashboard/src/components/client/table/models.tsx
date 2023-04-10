'use client'

import { jsonResponse, requireStatus, useApi } from "@/utils/app/apiConsumer";
import { SxProps, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Theme } from "@mui/material"
import { ModelInfo } from "@prisma/client"
import { useRouter } from 'next/navigation'


export default function ModelTable({ sx }: { sx?: SxProps<Theme> }) {
	const router = useRouter();

	const { data } = useApi({
		url: '/api/table/modelinfo',
		process: (response) => response.then(requireStatus(200)).then(jsonResponse<ModelInfo[]>)
	});

	const openModel = (model: ModelInfo) => (() => router.push(`/models/${model.ModelId}`))

	return (
		<TableContainer sx={sx}>
			<Table stickyHeader>
				<TableHead>
					<TableRow>
						<TableCell>Name</TableCell>
						<TableCell>Cube Type</TableCell>
						<TableCell>Epoch Count</TableCell>
						<TableCell>Git Hash</TableCell>
					</TableRow>
				</TableHead>
				<TableBody>
					{data?.map((model) => (
						<TableRow
							key={model.ModelId}
							hover
							onClick={openModel(model)}
							sx={{ cursor: 'pointer' }}
						>
							<TableCell>{model.ModelName}</TableCell>
							<TableCell>{model.CubeType}</TableCell>
							<TableCell>{model.EpochCount}</TableCell>
							<TableCell>{model.GitHash}</TableCell>
						</TableRow>
					))}
				</TableBody>
			</Table>
		</TableContainer>
	)
}

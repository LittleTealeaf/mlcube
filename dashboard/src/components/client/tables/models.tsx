'use client'

import { Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material"
import { ModelInfo } from "@prisma/client"
import { useRouter } from 'next/navigation'

export type ModelsTableParams = {
	models: ModelInfo[]
}

export default function ModelsTable({ models }: ModelsTableParams) {

	const router = useRouter()


	const openModel = (model: ModelInfo) => (() => router.push(`/models/${model.ModelId}`))

	return (
		<TableContainer sx={{maxHeight: '100%'}}>
			<Table stickyHeader>
				<TableHead>
					<TableRow>
						<TableCell>id</TableCell>
						<TableCell>Name</TableCell>
						<TableCell>Cube Type</TableCell>
						<TableCell>Epoch Count</TableCell>
						<TableCell>Git Hash</TableCell>
					</TableRow>
				</TableHead>
				<TableBody>
					{models.map((model) => (
						<TableRow
							key={model.ModelId}
							hover
							onClick={openModel(model)}
							sx={{cursor: 'pointer'}}
						>
							<TableCell>{model.ModelId}</TableCell>
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

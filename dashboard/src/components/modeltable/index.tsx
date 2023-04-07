'use client'

import { Button, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material"
import { Models } from "@prisma/client"
import Link from "next/link"

export type ModelTableProps = {
	models: Models[]
}

export default function ModelTable({ models }: ModelTableProps) {

	return (
		<TableContainer component={Paper}>
			<Table sx={{}} size="small" aria-label="simple table">
				<TableHead>
					<TableRow>
						<TableCell></TableCell>
						<TableCell align="center">ID</TableCell>
						<TableCell>Name</TableCell>
						<TableCell>Cube Type</TableCell>
						<TableCell>Git Hash</TableCell>
					</TableRow>
				</TableHead>
				<TableBody>
					{models.map((model) => (
						<TableRow
							hover
							key={model.ModelId}
							sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
						>
							<TableCell sx={{ p: 0, m: 0 }} align="right">
								<Button component={Link} href={`/models/${model.ModelId}`}>Open</Button>
							</TableCell>
							<TableCell align="center">{model.ModelId}</TableCell>
							<TableCell>{model.ModelName}</TableCell>
							<TableCell>{model.CubeType}</TableCell>
							<TableCell>{model.GitHash}</TableCell>
						</TableRow>
					))}
				</TableBody>
			</Table>
		</TableContainer>
	)
}

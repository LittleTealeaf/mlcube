'use client'

import { fetchAPI, jsonResponse, requireStatus } from "@/utils/fetch";
import { Paper, Table, TableBody, TableCell, TableContainer, TableHead, TablePagination, TableRow } from "@mui/material";
import { Epochs } from "@prisma/client";
import React, { ChangeEvent, useCallback, useEffect } from "react";
import { useState } from "react";
import useSWR from 'swr';



export type EpochsTableParams = {
	ModelId: number
}


export default function EpochTable({ ModelId }: EpochsTableParams) {

	const [page, setPage] = useState(0);
	const [perPage, setPerPage] = useState(10);
	const [epochs, setEpochs] = useState<Epochs[] | null>(null);


	useEffect(() => {
		setEpochs(null);
		fetchAPI("/api/epochs/table/paginated", { per_page: perPage, page, model_id: ModelId }).then(jsonResponse).then(setEpochs)
	}, [perPage, page])

	const { data: count, mutate, isValidating } = useSWR<number>(
		'api/epochs/count',
		() => fetchAPI('/api/epochs/count', { ModelId }, {next: {revalidate: 60}})
			.then(requireStatus(200))
			.then(jsonResponse)
	)

	const handleChangePerPage = useCallback((event: ChangeEvent<HTMLInputElement>) => {
		const updatedRowsPerPage = parseInt(event.target.value, 10);
		setPerPage(updatedRowsPerPage);
	}, []);

	const handleChangePage = useCallback((event: unknown, newPage: number) => {
		setPage(newPage);
	}, []);

	return (
		<>
			<TablePagination
				rowsPerPageOptions={[10, 25, 50, 100, 200, 500, 1000]}
				component="div"
				count={count || 0}
				rowsPerPage={perPage}
				page={page}
				onRowsPerPageChange={handleChangePerPage}
				onPageChange={handleChangePage}
				sx={{ position: 'sticky' }}
			/>
			<TableContainer sx={{ maxHeight: '100vh' }}>
				<Table size="small" stickyHeader>
					<TableHead>
						<TableRow>
							<TableCell>Epoch</TableCell>
							<TableCell>Loss</TableCell>
							<TableCell>Reward</TableCell>
						</TableRow>
					</TableHead>
					<TableBody>
						{
							epochs != null && epochs.map((epoch) => (
								<TableRow key={epoch.EpochId}>
									<TableCell>{epoch.Epoch}</TableCell>
									<TableCell>{epoch.Loss}</TableCell>
									<TableCell>{epoch.Reward}</TableCell>
								</TableRow>
							)) || Array(perPage).map((_, index) => (
								<TableRow key={index}>
									<TableCell></TableCell>
									<TableCell></TableCell>
									<TableCell></TableCell>
								</TableRow>
							))
						}
					</TableBody>
				</Table>
			</TableContainer>
		</>
	)
}

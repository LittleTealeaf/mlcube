'use client'

import { AppBar, Toolbar, Typography } from "@mui/material"


export type ModelNavBarParams = {
	ModelName?: string
}


export default function ModelNavBar({ ModelName }: ModelNavBarParams) {


	return (
		<>
			<AppBar position="static">
				<Toolbar variant="dense">
					<Typography variant="subtitle1" sx={{ flexGRow: 1 }}>{ModelName}</Typography>
				</Toolbar>
			</AppBar>
		</>
	)
}

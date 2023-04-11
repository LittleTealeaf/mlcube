'use client'

import { Button } from "@mui/material"


export default function NavBar({ }) {


	return (
		<div className="flex p-3">
			<h2 className="font-bold text-lg grow">
				MlCube
			</h2>
			<Button href="/">Home</Button>
			<Button href="/models">Models</Button>
		</div>
	)
}

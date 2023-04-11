'use client'

import { AppBar, Button, Container, Divider, Toolbar, Typography } from "@mui/material"

const navElements = [
	{
		name: "Home",
		path: "/"
	},
	{
		name: "Models",
		path: "/models"
	}
];

export default function NavBar({ }) {
	return (
		<AppBar position="static">
			<Container maxWidth="xl">
				<Toolbar disableGutters sx={{ display: 'flex' }}>
					<Typography variant="h6" color="inherit" >
						MlCube
					</Typography>
					<Divider orientation="vertical" flexItem sx={{ m: '10px' }} />
					{
						navElements.map(({ name, path }) => (
							<Button href={path} key={path} color="inherit">
								{name}
							</Button>
						))
					}
				</Toolbar>
			</Container>
		</AppBar>
	)
}

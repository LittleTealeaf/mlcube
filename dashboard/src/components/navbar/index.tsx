'use client'

import { AppBar, Box, Breadcrumbs, Button, Link as MuiLink, Toolbar, Typography } from "@mui/material"
import Link from "next/link";
import { usePathname } from 'next/navigation';

export type NavBarParams = {

}

export default function NavBar({ }) {

	const path = usePathname().split("/").map(path => ({ path, link: path }));

	for (var i = 0; i < path.length - 1; i++) {
		path[i + 1].link = path[i].link.concat("/").concat(path[i + 1].link);
	}

	path[0].path = "MLCube";


	if (path[path.length - 1].path == "") {
		path.pop();
	}


	return (
		<Box sx={{ flexGrow: 1 }}>
			<AppBar position="static">
				<Toolbar>
					<Breadcrumbs sx={{ flexGrow: 1 }}>
						{path.map(({ path, link }) => (
							<MuiLink href={link} color="inherit" underline="hover" component={Link}>
								<Typography color="white" variant="h6">
									{path}
								</Typography>
							</MuiLink>
						))}
					</Breadcrumbs>
					<Button color="inherit" component={Link} href="/">Home</Button>
					<Button color="inherit" component={Link} href="/models">Models</Button>
				</Toolbar>
			</AppBar>
		</Box>
	)
}

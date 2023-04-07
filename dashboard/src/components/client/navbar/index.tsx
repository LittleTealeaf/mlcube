'use client'

import { AppBar, Box, Breadcrumbs, Button, Toolbar, Typography } from "@mui/material"
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


//TODO: redo the header to be ontop on all parts of the page. We can do this by only including a scrollbar on content

	return (
		<>
			<AppBar position="static">
				<Toolbar>
					<Breadcrumbs sx={{ flexGrow: 1 }}>
						{path.map(({ path, link }, index) => (
							<Button key={index} component={Link} href={link} sx={{ color: 'white', padding: 0, margin: 0, width: 'fit-content' }}>
								<Typography sx={{ fontSize: "15px", fontWeight: "bolder", width: 'fit-content', margin: 0, padding: 0 }}>
									{path}
								</Typography>
							</Button>
						))}
					</Breadcrumbs>
					<Button color="inherit" component={Link} href="/">Home</Button>
					<Button color="inherit" component={Link} href="/models">Models</Button>
				</Toolbar>
			</AppBar>
		</>
	)
}

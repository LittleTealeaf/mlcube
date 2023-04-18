"use client";

import { AppBar, Box, Button, Toolbar, Typography } from "@mui/material";

export default function NavBar({}) {
  return (
    <AppBar position="static">
      <Toolbar>
        <Box sx={{ display: "flex", width: "100%" }}>
          <Typography variant="h6">MlCube</Typography>
          <div className="flexgrow"></div>
          <Button color="inherit" href="/">
            Home
          </Button>
          <Button color="inherit" href="/models">
            Models
          </Button>
        </Box>
      </Toolbar>
    </AppBar>
  );
}

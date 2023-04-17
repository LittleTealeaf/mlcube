"use client";

import { Button, IconButton, Typography } from "@mui/material";
import { Model } from "@prisma/client";
import { GitHub as GitHubIcon } from "@mui/icons-material";

export type ModelNavBarParams = {
  model: Model;
};

export default function ModelNavBar({ model }: ModelNavBarParams) {
  return (
    <div className="flex p-2">
      <div style={{ flexGrow: 1, display: 'flex', }}>
				<Typography variant="h6">{model.ModelName}</Typography>
      </div>
      <Button href={`/models/${model.ModelId}/`} size="small">
        Dashboard
      </Button>
      <Button href={`/models/${model.ModelId}/live`} size="small">
        Live
      </Button>
			{model.GitHash && <IconButton href={`https://www.github.com/LittleTealeaf/mlcube/tree/${model.GitHash}`}><GitHubIcon/></IconButton>}
    </div>
  );
}

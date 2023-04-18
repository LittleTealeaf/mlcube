"use client";
import { GitHub as GitHubIcon } from "@mui/icons-material";
import { Button, IconButton, Typography } from "@mui/material";
import { Model } from "@prisma/client";

export type Props = {
  model: Model;
};

export default function ModelNavBar({ model }: Props) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "row",
        padding: "2px",
      }}
    >
      <Typography variant="h6" sx={{ flexGrow: 1 }}>
        {model.ModelName}
      </Typography>
      <Button href={`/models/${model.ModelId}`}>Dashboard</Button>
      {model.GitHash && (
        <IconButton
          href={`https://www.github.com/LittleTealeaf/mlcube/tree/${model.GitHash}`}
        >
          <GitHubIcon />
        </IconButton>
      )}
    </div>
  );
}

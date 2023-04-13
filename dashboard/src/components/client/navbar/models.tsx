"use client";

import { Button } from "@mui/material";
import { Model } from "@prisma/client";

export type ModelNavBarParams = {
  model: Model;
};

export default function ModelNavBar({ model }: ModelNavBarParams) {
  return (
    <div className="flex p-2">
      <h4 className="grow font-bold">{model.ModelName}</h4>
      <Button href={`/models/${model.ModelId}/`} size="small">
        Dashboard
      </Button>
      <Button href={`/models/${model.ModelId}/live`} size="small">
        Live
      </Button>
    </div>
  );
}

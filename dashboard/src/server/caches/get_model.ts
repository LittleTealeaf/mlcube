"server-only";
import { prisma } from "@/database";
import { cache } from "react";

export const getModel = cache(async (ModelId: string) => {
  const model = await prisma.model.findFirst({
    where: {
      ModelId: {
        equals: Number(ModelId),
      },
    },
  });
  return model;
});

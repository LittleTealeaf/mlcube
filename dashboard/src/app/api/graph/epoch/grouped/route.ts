import { prisma } from "@/database";
import { getParameters } from "@/server/api/params";
import { ApiGraphEpochGrouped } from "@/types/api";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const { ModelId } = getParameters<ApiGraphEpochGrouped>(request);

  const id = await prisma.model
    .findFirstOrThrow({
      where: {
        ModelId: {
          equals: Number(ModelId),
        },
      },
      select: {
        ModelName: true,
      },
    })
    .then(({ ModelName }) => ModelName)
    .catch(() => undefined);

  const data = await prisma.groupedEpoch
    .findMany({
      where: {
        ModelId: {
          equals: Number(ModelId),
        },
      },
      select: {
        EpochGroup: true,
        AvgLoss: true,
      },
      orderBy: {
        EpochGroup: "asc",
      },
    })
    .then((data) =>
      data.map(({ EpochGroup, AvgLoss }) => ({
        x: EpochGroup || 0,
        y: AvgLoss || 0,
      }))
    )
    .catch(() => undefined);

  return NextResponse.json(
    {
      id,
      data,
    },
    {
      status: (id && data && 200) || 500,
    }
  );
}

import { prisma } from "@/database";
import { ApiTableModelInfo } from "@/types/api";
import { getParameters } from "@/utils/api/parameters";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const params = getParameters<ApiTableModelInfo>(request);

  const page = params.page && Number(params.page);
  const perPage = params.perPage && Number(params.perPage);

  const models = await prisma.modelInfo.findMany({
    take: page && perPage,
    skip: perPage && page && page * perPage,
    orderBy: {
      ModelId: params.sort || "desc",
    },
  });

  return NextResponse.json(models);
}

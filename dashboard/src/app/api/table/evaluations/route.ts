import { prisma } from "@/database";
import { getParameters } from "@/server/api/params";
import { ApiTableEvaluations } from "@/types/api";
import { equal } from "assert";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
  const params = getParameters<ApiTableEvaluations>(request);

  const perPage = Number(params.PerPage) || 25;
  const page = Number(params.Page) || 0;

  const data = await Promise.all(
    await prisma.evaluationInfo
      .findMany({
        where: {
          ModelId: {
            equals: Number(params.ModelId),
          },
        },
        select: {
          EvaluationId: true,
          ModelId: true,
          Epoch: true,
          AvgReward: true,
          MaxReward: true,
          MinReward: true,
          FinalReward: true,
          MoveCount: true,
          Solved: true,
        },
        take: perPage,
        skip: page * perPage,
      })
      .then((data) =>
        data.map((entry) =>
          prisma.evaluationMove
            .findMany({
              where: {
                EvaluationId: {
                  equals: entry.EvaluationId,
                },
              },
              select: {
                MoveName: true,
              },
              take: perPage,
              skip: page * perPage,
            })
            .then((moves) => moves.map((move) => move.MoveName))
            .then((moves) => ({
              ...entry,
							EvaluationId: undefined,
              moves,
            }))
        )
      )
  );

  return NextResponse.json(data);
}

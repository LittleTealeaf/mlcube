import { prisma } from "@/database"
import { GraphEpochParams } from "@/types/apitypes"
import { getParameters } from "@/utils/api/parameters"
import { NextResponse } from "next/server";



export async function GET(request: Request) {
	const { ModelId } = getParameters<GraphEpochParams>(request)

	const epochs = await prisma.groupedEpoch.findMany({
		where: {
			ModelId: {
				equals: Number(ModelId)
			}
		},
		select: {
			EpochCategory: true,
			AvgReward: true
		}
	});

	const { ModelName: id } = await prisma.model.findFirstOrThrow({
		where: {
			ModelId: {
				equals: Number(ModelId)
			},
		},
		select: {
			ModelName: true
		}
	})

	const data = epochs.map(({ EpochCategory, AvgReward }) => ({ x: EpochCategory, y: AvgReward }));

	return NextResponse.json({
		id,
		data
	})
}

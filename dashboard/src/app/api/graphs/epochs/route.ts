import { prisma } from "@/database";
import { GraphEpochParams } from "@/types/apitypes";
import { getParameters } from "@/utils/api/parameters";
import { NextResponse } from "next/server";


export async function GET(request: Request) {
	const { ModelId } = getParameters<GraphEpochParams>(request);

	const epochs = await prisma.groupedEpoch.findMany({
		where: {
			ModelId: {
				equals: Number(ModelId)
			}
		},
		select: {
			ModelId: false,
			AvgLoss: true,
			AvgReward: true,
			GroupedId: false,
			EpochCategory: true
		}
	})

	// const values = epochs.map(({ AvgLoss, EpochCategory, AvgReward }) => ({
	// 	x: EpochCategory,
	// 	reward: AvgReward,
	// 	loss: AvgLoss
	// }));


	return NextResponse.json({ModelId})
}

import { prisma } from "@/database";
import { GraphEpochParams } from "@/types/apitypes";
import { getParameters } from "@/utils/api/parameters";
import { NextResponse } from "next/server";


export async function GET(request: Request) {

	const params = getParameters<GraphEpochParams>(request);

	const data = params.filter == 'recent' ? getRecent(params) : getAll(params);

	const model = prisma.model.findFirstOrThrow({
		where: {
			ModelId: Number(params.modelid)
		},
		select: {
			ModelName: true
		}
	}).then(({ ModelName }) => ModelName);

	return NextResponse.json({
		id: await model,
		data: await data
	})
}


async function getAll({ modelid, select }: GraphEpochParams) {
	const data = await prisma.groupedEpoch.findMany({
		where: {
			ModelId: {
				equals: Number(modelid)
			}
		},
		select: {
			EpochGroup: true,
			AvgReward: select == 'reward',
			AvgLoss: select == 'loss',
		},
		orderBy: {
			EpochGroup: 'asc'
		}
	});

	return data.map(
		({ EpochGroup, AvgReward, AvgLoss }) => ({ x: EpochGroup, y: AvgReward || AvgLoss || 0 })
	);
}

async function getRecent({ modelid, select, count }: GraphEpochParams) {
	const data = await prisma.epoch.findMany({
		where: {
			ModelId: {
				equals: Number(modelid)
			}
		},
		select: {
			Epoch: true,
			Reward: select == 'reward',
			Loss: select == 'loss',
		},
		orderBy: {
			Epoch: 'desc'
		},
		take: Number(count) || 100
	});

	return data.map(
		({ Epoch, Reward, Loss }) => ({ x: Epoch, y: Loss || Reward })
	);
}


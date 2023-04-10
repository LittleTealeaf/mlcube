import { prisma } from "@/database";
import { ApiGraphEpoch } from "@/types/api";
import { getParameters } from "@/utils/api/parameters";
import { NextResponse } from "next/server";


// TODO: Refactor this to a new api endpoint

export async function GET(request: Request) {

	const params = getParameters<ApiGraphEpoch>(request);

	const datapromise = params.filter == 'recent' ? getRecent(params) : getAll(params);

	const namepromise = prisma.model.findFirstOrThrow({
		where: {
			ModelId: Number(params.modelid)
		},
		select: {
			ModelName: true
		}
	})
		.then(({ ModelName }) => ModelName)
		.catch(() => null);

	const data = await datapromise;
	const name = await namepromise;

	return NextResponse.json({
		id: name,
		data
	}, {
		status: data && name ? 200 : 500
	})
}


async function getAll({ modelid, select }: ApiGraphEpoch['params']) {
	try {
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
			({ EpochGroup, AvgReward, AvgLoss }) => ({ x: EpochGroup || -1, y: AvgReward || AvgLoss || 0 })
		);
	} catch {
		return null;
	}
}

async function getRecent({ modelid, select, count }: ApiGraphEpoch['params']) {
	try {
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
			({ Epoch, Reward, Loss }) => ({ x: Epoch || -1, y: Loss || Reward || 0 })
		);
	} catch {
		return null;
	}
}


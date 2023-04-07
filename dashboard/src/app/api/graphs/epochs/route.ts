import { prisma } from "@/db";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
	
	const { searchParams } = new URL(request.url);

	const ModelId = Number(searchParams.get('ModelId'));

	const epochs = await prisma.epochs.findMany({
		where: {
			ModelId: {
				equals: ModelId,
			},
		},
		select: {
			Epoch: true,
			Loss: true,
			Reward: true,
		},

	})


	// TODO: Create a Table View that groups epochs by epoch number group and ModelId to pull for this table

	return NextResponse.json(epochs);
}

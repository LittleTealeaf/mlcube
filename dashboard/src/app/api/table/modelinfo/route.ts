import { prisma } from "@/database";
import { NextResponse } from "next/server";


export async function GET() {


	const models = await prisma.modelInfo.findMany({
		orderBy: {
			ModelId: 'desc'
		}
	});

	return NextResponse.json(models);
}

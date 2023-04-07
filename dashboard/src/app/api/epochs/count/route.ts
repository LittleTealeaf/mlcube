import { prisma } from "@/db";
import { NextResponse } from "next/server";

export async function GET(request: Request) {
	const {searchParams} = new URL(request.url);
	const model_id = Number(searchParams.get("ModelId"));

	const count = await prisma.epochs.count({
		where: {
			ModelId: {
				equals: model_id
			}
		}
	});

	return NextResponse.json(count);
}

import { prisma } from "@/db";
import { NextResponse } from "next/server";



export async function GET(request: Request) {
	const {searchParams} = new URL(request.url);
	const per_page = Number(searchParams.get("per_page"))
	const page = Number(searchParams.get("page"));
	const model_id = Number(searchParams.get("model_id"));

	const results = await prisma.epochs.findMany({
		where: {
			ModelId: {
				equals: model_id
			}
		},
		skip: per_page * page,
		take: per_page,
		orderBy: {
			Epoch: 'asc'
		}
	})

	return NextResponse.json(results);
}

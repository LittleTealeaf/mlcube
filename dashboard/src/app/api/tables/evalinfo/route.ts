import { prisma } from "@/database";
import { TableEvalInfoParams } from "@/types/apitypes";
import { getParameters } from "@/utils/api/parameters";
import { NextResponse } from "next/server";


// TODO: Refactor this to a new api endpoint

export async function GET(request: Request) {

	const params = getParameters<TableEvalInfoParams>(request);

	const page = params.page && Number(params.page);
	const perPage = params.perPage && Number(params.perPage);

	const data = await prisma.evaluationInfo.findMany({
		where: {
			ModelId: {
				equals: Number(params.modelid)
			}
		},
		orderBy: {
			Epoch: params.sort || 'desc'
		},
		take: page && perPage,
		skip: perPage && page && page * perPage
	}).catch(() => null);

	return NextResponse.json(data, {
		status: data ? 200 : 500
	});
}

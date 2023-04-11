import { prisma } from "@/database";

export type ModelIdRoute = {
	params: {
		ModelId: string;
	}
}


export async function getModelRoute(params: ModelIdRoute) {
	return await prisma.model.findFirstOrThrow({
		where: {
			ModelId: {
				equals: Number(params.params.ModelId)
			}
		}
	})
}

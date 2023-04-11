import { EpochGraph } from "@/components/client/graphs/epochs";
import { prisma } from "@/database"
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";
import { notFound } from "next/navigation";


export default async function Page(params: ModelIdRoute) {
	const model = await getModelRoute(params) || notFound();

	return (
		<>
			<title>{model.ModelName}</title>
			<EpochGraph
				sx={{ height: '400px', m: '10px auto', width: '90%' }}
				params={{ modelid: model.ModelId, select: 'reward' }}
			/>
			<EpochGraph
				sx={{ height: '400px', m: '10px auto', width: '90%' }}
				params={{ modelid: model.ModelId, select: 'loss' }}
			/>
		</>
	)
}

export async function generateStaticParams() {
	const models = await prisma.model.findMany();

	return models.map(({ ModelId }) => ({ ModelId: String(ModelId) }));
}

import { prisma } from "@/db";

export default async function Page({ params }: { params: { ModelId: number } }) {

	const model = await prisma.models.findFirst({
		where: {
			ModelId: {
				equals: Number(params.ModelId)
			}
		}
	});

	if (!model) {
		return null;
	}

	return (
		<div>
			{model.ModelName}
		</div>
	)
}


export async function generateStaticParams() {
	return await prisma.models.findMany().then(models => models.map(({ ModelId }) => String(ModelId)).map(ModelId => ({ ModelId })))
}

import { prisma } from "@/database"

type PageParams = {
	params: {
		ModelId: string
	}
}

export default async function Page({ params }: PageParams) {
	const model = await prisma.model.findFirstOrThrow({ where: { ModelId: { equals: Number(params.ModelId) } } });


	return (
		<>
			<title>{model.ModelName}</title>
		</>
	)
}

export async function generateStaticParams() {
	const models = await prisma.model.findMany();

	return models.map(({ ModelId }) => ({ ModelId: String(ModelId) }));
}

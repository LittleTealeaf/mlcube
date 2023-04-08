import ModelsTable from "@/components/client/tables/models";
import { prisma } from "@/database"


export default async function Page({ }) {

	const models = await prisma.modelInfo.findMany({
		orderBy: {
			ModelId: 'desc'
		}
	});

	return (
		<div style={{ height: '500px' }}>
			<ModelsTable {...{ models }} />
		</div>
	)
}

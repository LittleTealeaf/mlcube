import ModelsTable from "@/components/client/tables/models";
import { prisma } from "@/database"


// TODO: Add search filter

export default async function Page({ }) {

	const models = await prisma.modelInfo.findMany({
		orderBy: {
			ModelId: 'desc'
		}
	});

	return (
		<>
			<title>Models</title>
			<ModelsTable {...{ models }} sx={{ flexGrow: 1 }} />
		</>
	)
}

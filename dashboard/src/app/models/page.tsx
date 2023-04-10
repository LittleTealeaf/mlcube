import ModelTable from "@/components/client/table/models"
import { prisma } from "@/database"


// TODO: Add search filter

export default async function Page({ }) {
	//
	// const models = await prisma.modelInfo.findMany({
	// 	orderBy: {
	// 		ModelId: 'desc'
	// 	}
	// });

	return (
		<>
			<title>Models</title>
			<ModelTable />
		</>
	)
}

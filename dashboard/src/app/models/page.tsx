import ModelTable from "@/components/client/table/models"
import { prisma } from "@/database"


// TODO: Add search filter

export default async function Page({ }) {

	return (
		<>
			<title>Models</title>
			<ModelTable />
		</>
	)
}

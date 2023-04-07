import ModelTable from "@/components/modeltable";
import { prisma } from "@/db"



export default async function Page({ }) {

	const models = await prisma.models.findMany();


	return (
		<div>
			<ModelTable models={models} />
		</div>
	)
}

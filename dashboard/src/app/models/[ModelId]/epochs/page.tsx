import EpochTable from "@/components/client/epochs/table";
import { prisma } from "@/db";



export default function Page({ params }: { params: { ModelId: number } }) {




	return (
		<>
			<EpochTable ModelId={params.ModelId} />
		</>
	)
}


export async function generateStaticParams() {
	return await prisma.models.findMany().then(models => models.map(({ ModelId }) => String(ModelId)).map(ModelId => ({ ModelId })))
}

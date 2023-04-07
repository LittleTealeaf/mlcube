import EpochTable from "@/components/client/epochs/table";



export default function Page({ params }: { params: { ModelId: number } }) {




	return (
		<>
			<EpochTable ModelId={params.ModelId} />
		</>
	)
}

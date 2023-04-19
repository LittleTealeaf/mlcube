import { getModel } from "@/server/caches/get_model";
import { WithModelId } from "@/types/props";
import { notFound } from "next/navigation";
import GraphEpochGrouped from "@/components/GraphEpochGrouped";

export default async function Page({ params: { ModelId } }: WithModelId) {
	const model = (await getModel(ModelId)) || notFound();

	return (
		<>
			<title>{model.ModelName}</title>
			<GraphEpochGrouped
				params={{ ModelId: String(model.ModelId) }}
				sx={{ height: "300px", width: "100%" }}
			/>
		</>
	);
}

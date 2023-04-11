import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";
import { notFound } from "next/navigation";


export default async function Page(params: ModelIdRoute) {

	const model = await getModelRoute(params) || notFound();

	return (
		<>
			<title>{`Live: ${model.ModelName}`}</title>
		</>
	)
}

import ModelNavBar from "@/components/client/navbar/models";
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";
import { notFound } from "next/navigation";


export default async function Layout(params: { children: React.ReactNode } & ModelIdRoute) {

const model = await getModelRoute(params) || notFound();

	return (
		<div>
			<ModelNavBar model={model} />
			{params.children}
		</div>
	)
}

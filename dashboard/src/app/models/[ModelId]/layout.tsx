import ModelNavBar from "@/components/client/navbar/models";
import { prisma } from "@/database";
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";


export default async function Layout(params: { children: React.ReactNode } & ModelIdRoute) {

const model = await getModelRoute(params);

	return (
		<div>
			<ModelNavBar model={model} />
			{params.children}
		</div>
	)
}

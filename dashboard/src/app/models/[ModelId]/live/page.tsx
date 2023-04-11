import { EpochGraph } from "@/components/client/graphs/epochs";
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";


export default async function Page(params: ModelIdRoute) {

	const model = await getModelRoute(params);

	return (
		<>
			<title>{`Live: ${model.ModelName}`}</title>
			<div>
				<div className="w-screen h-[500px] flex">
					<div className="flex flex-col w-1/2">
						<EpochGraph filter="recent" count={50} modelid={Number(params.params.ModelId)} select="loss" sx={{ height: '50%', margin: '5px' }} />
						<EpochGraph filter="recent" count={50} modelid={Number(params.params.ModelId)} select="reward" sx={{ height: '50%', margin: '5px' }} />
					</div>
					<div>awef</div>
				</div>
			</div>
		</>
	)
}

import { EpochGraph } from "@/components/client/graphs/epochs";
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";
import { notFound } from "next/navigation";


export default async function Page(params: ModelIdRoute) {

	const model = await getModelRoute(params) || notFound();

	return (
		<>
			<title>{`Live: ${model.ModelName}`}</title>
			<div>
				<div style={{
					width: '100vw',
					height: '500px',
					display: 'flex',
					flexWrap: 'wrap'
				}}>
					<div style={{
						display: 'flex',
						flexDirection: 'column',
						width: '50%'
					}}>
						<EpochGraph filter="recent" count={50} modelid={Number(params.params.ModelId)} select="loss" sx={{
							height: '50%',
							margin: '5px'
						}} />
						<EpochGraph filter="recent" count={50} modelid={Number(params.params.ModelId)} select="reward" sx={{
							height: '50%',
							margin: '5px'
						}} />
					</div>
					<div>awef</div>
				</div>
			</div>
		</>
	)
}

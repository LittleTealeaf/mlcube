import { EpochGraph } from "@/components/client/graphs/epochs";
import { ModelIdRoute, getModelRoute } from "@/utils/app/route_params";
import { notFound } from "next/navigation";

export default async function Page(params: ModelIdRoute) {
  const model = (await getModelRoute(params)) || notFound();

  return (
    <>
      <title>{`Live: ${model.ModelName}`}</title>
			<EpochGraph params={{modelid: model.ModelId, select: 'loss', filter: 'recent', count: 100}} sx={{height: '400px'}} refresh_interval={1000}/>
    </>
  );
}

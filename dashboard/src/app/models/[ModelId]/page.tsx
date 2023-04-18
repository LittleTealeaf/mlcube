import { getModel } from "@/server/caches/get_model";
import { WithModelId } from "@/types/props";
import { notFound } from "next/navigation";

export default async function Page({ params: { ModelId } }: WithModelId) {
  const model = (await getModel(ModelId)) || notFound();

  return <></>;
}

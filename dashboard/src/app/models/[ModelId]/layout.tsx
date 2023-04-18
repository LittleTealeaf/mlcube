import { PropsWithChildren } from "react";
import { WithModelId } from "@/types/props";
import { getModel } from "@/server/caches/get_model";
import { notFound } from "next/navigation";
import ModelNavBar from "@/components/ModelNavBar";

export default async function Layout({
  children,
  params: { ModelId },
}: PropsWithChildren & WithModelId) {
  const model = (await getModel(ModelId)) || notFound();

  return (
    <>
      <ModelNavBar model={model} />
      <hr />
      {children}
    </>
  );
}

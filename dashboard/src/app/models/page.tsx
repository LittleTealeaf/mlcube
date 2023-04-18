import ModelTable from "@/components/ModelTable";
import { prisma } from "@/database";

export default async function Page({}) {
  const models = await prisma.modelInfo.findMany({
    orderBy: {
      ModelId: "desc",
    },
  });

  return (
    <>
      <title>Models</title>
      <ModelTable models={models} />
    </>
  );
}

import ModelInfoTable from "@/components/static/modelInfoTable";
import { prisma } from "@/database";

// TODO: Add search filter

export const revalidate = 60;

async function getModelInfo() {
  const models = await prisma.modelInfo.findMany({
    orderBy: {
      ModelId: "desc",
    },
  });
  return models;
}

export default async function Page({}) {
  const modelinfo = await getModelInfo();

  return (
    <>
      <title>Models</title>
      <ModelInfoTable modelinfo={modelinfo} />
    </>
  );
}

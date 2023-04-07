import ModelNavBar from "@/components/client/navbar/models"
import { prisma } from "@/db"
import { ReactNode } from "react"

type LayoutParams = {
	children: ReactNode,
	params: {
		ModelId: number
	}
}

export default async function Layout({ children, params: { ModelId } }: LayoutParams) {

	const model = await prisma.models.findFirst({ where: { ModelId: { equals: Number(ModelId) } } });

	return (
		<>
			<ModelNavBar ModelName={model?.ModelName || undefined} />
			{children}
		</>
	)
}

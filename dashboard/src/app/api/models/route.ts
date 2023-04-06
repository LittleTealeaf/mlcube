import { prisma } from "@/db";
import { NextResponse } from "next/server";

export async function GET() {
	const models = await prisma.models.findMany();
	return NextResponse.json({ models });
}

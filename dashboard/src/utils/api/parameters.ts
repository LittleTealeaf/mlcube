

export function getParameters<T = { [key: string]: string }>(request: Request): T {
	const { searchParams } = new URL(request.url);
	const parameters = searchParams.entries();
	return Object.fromEntries(parameters) as T;
}

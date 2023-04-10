import useSWR from 'swr';

export function buildURL(url: string, params?: { [k: string]: any }) {
	const parameters = Object.entries(params || {}).map(([key, value]) => `${key}=${String(value)}`).join("&");
	return `${url}${parameters.length > 0 && '?' || ''}${parameters}`
}

export function getApi<T = { [k: string]: any }>(url: string, params?: T, init?: RequestInit): Promise<Response> {
	return fetch(buildURL(url, params || undefined), init);
}


export function requireStatus(statusCode: number) {
	return async (response: Response): Promise<Response> => (
		new Promise((resolve, reject) => {
			if (response.status != statusCode) {
				reject(`Expected Status ${statusCode}, found ${response.status}`);
			} else {
				resolve(response);
			}
		})
	)
}

export function jsonResponse<T>(response: Response): Promise<T> {
	return response.json()
}

type UseApiParams<T> = {
	url: string,
	params?: { [key: string]: any },
	init?: RequestInit,
	process: (response: Promise<Response>) => Promise<T>
}

export function useApi<T>({ url, params, init, process }: UseApiParams<T>) {
	return useSWR(buildURL(url, params), () => process(getApi(url, params, init)))
}

import { ApiType } from '@/types/api';
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

export type UseApiParameters<T extends ApiType> = {
	url: T['url'];
	params: T['params'];
	init?: RequestInit
}

export function useApi<T extends ApiType>({ url, params, init }: UseApiParameters<T>) {
	const fetch_url = buildURL(url, params);
	return useSWR(fetch_url, () => fetch(fetch_url, init).then(requireStatus(200)).then(jsonResponse<T['response']>))
}

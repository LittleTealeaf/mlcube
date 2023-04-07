
export type RestEndpoint = 'GET' | 'PUT' | 'POST' | 'DELETE';

export type ApiParams = {
	[key: string]: any;
}

export async function fetchAPI(url: string, params: ApiParams = {}, init?: RequestInit) {
	const fetch_url = `${url}?${Object.entries(params).map(([key,value]) => `${key}=${value}`).join("&")}`

	return fetch(fetch_url, init);
}


export function requireStatus(statusCode: number) {
	return async (response: Response): Promise<Response> => new Promise((resolve, reject) => {
		if(response.status != statusCode) {
			reject(`Expected Status ${statusCode}, found ${response.status}`);
		} else {
			resolve(response);
		}
	})
}


export async function jsonResponse(response: Response) {
	return response.json()
}

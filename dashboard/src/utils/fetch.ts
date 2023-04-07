
export type RestEndpoint = 'GET' | 'PUT' | 'POST' | 'DELETE';

export type ApiParams = {
	[key: string]: any;
}

export async function fetchAPI(url: string, method: RestEndpoint, params: ApiParams = {}) {
	// const fetch_url = new URL(url);
	// Object.entries(params).forEach(([key, value]) => {
	// 	fetch_url.searchParams.set(key, value);
	// })
	
	const fetch_url = `${url}?${Object.entries(params).map(([key,value]) => `${key}=${value}`).join("&")}`

	return fetch(fetch_url, {method});
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

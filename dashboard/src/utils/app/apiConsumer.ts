

export function getApi<T = { [k: string]: any }>(url: string, params?: T, init?: RequestInit): Promise<Response> {
	const parameters = Object.entries(params || {}).map(([key, value]) => `${key}=${String(value)}`).join("&");
	const fetch_url = `${url}${parameters.length > 0 && '?' || ''}${parameters}`


	return fetch(fetch_url, init);
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


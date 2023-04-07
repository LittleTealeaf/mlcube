export function equalsParameter<T>(value: T) {
	return value != undefined ? { equals: value } : undefined;
}

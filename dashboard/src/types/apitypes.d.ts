
export type GraphEpochParams = {
	modelid: number;
	select: 'loss' | 'reward';
	filter?: 'all' | 'recent';	
	count?: number;
}


export type GraphEpochResult = {
	id: string,
	data: {
		x: number,
		y: number
	}[]
}


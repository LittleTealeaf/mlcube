import { EvaluationInfo } from "@prisma/client";

export type ApiType = {
  url: string;
  params?: { [key: string]: any };
  response: any;
};

export type ApiGraphEpochGrouped = {
  url: "/api/graph/epoch/grouped";
  params: {
    ModelId: string;
  };
  response: {
    id: string;
    data: {
      x: number;
      y: number;
    }[];
  };
};

export type ApiTableEvaluations = {
	url: "/api/table/evaluations";
	params: {
		ModelId: string;
		PerPage?: number;
		Page?: number;
	},
	response: (EvaluationInfo & {
		moves: string[];
	})[]
}

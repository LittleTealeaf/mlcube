import { EvaluationInfo, ModelInfo } from "@prisma/client";

export type ApiType = {
  url: string;
  params?: { [key: string]: any };
  response: any;
};

export type ApiTableModelInfo = {
  url: "/api/table/modelinfo";
  params: {
    page?: number;
    perPage?: number;
    sort?: "asc";
  };
  response: ModelInfo[];
};

export type ApiTableEvalInfo = {
  url: "/api/table/evalinfo";
  params: {
    modelid: number;
    page?: number;
    perPage?: number;
    sort?: "asc" | "desc";
  };
  response: EvaluationInfo[];
};

export type GraphResponse = {
  id: string;
  data: {
    x: number;
    y: number;
  }[];
};

export type ApiGraphEpoch = {
  url: "/api/graph/epoch";
  params: {
    modelid: number;
    select: "loss" | "reward";
    filter?: "all" | "recent";
    count?: number;
  };
  response: GraphResponse;
};

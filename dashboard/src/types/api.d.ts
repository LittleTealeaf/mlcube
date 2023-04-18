export type ApiType = {
  url: string;
  params?: { [key: string]: any };
  response: any;
};

export type GraphEpochGrouped = {
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

import { SxProps, Theme } from "@mui/material";
import { CSSProperties } from "react";

export type WithModelId = {
	params: {
		ModelId: string;
	};
};

export type WithStyle = {
	style?: CSSProperties;
};

export type WithSx = {
	sx?: SxProps<Theme>;
};

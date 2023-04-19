"use client";

import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@mui/material";
import { ModelInfo } from "@prisma/client";
import { useRouter } from "next/navigation";

export type Props = {
  models?: ModelInfo[];
};

// TODO: Add Evaluation Count

export default function ModelTable({ models }: Props) {
  const router = useRouter();

  const openModel = (model: ModelInfo) => () =>
    router.push(`/models/${model.ModelId}`);

  return (
    <TableContainer>
      <Table stickyHeader>
        <TableHead>
          <TableRow>
            <TableCell sx={{fontWeight: 'bold'}}>Name</TableCell>
            <TableCell sx={{fontWeight: 'bold'}}>Cube Type</TableCell>
            <TableCell sx={{fontWeight: 'bold'}}>Epoch Count</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {models?.map((model) => (
            <TableRow
              key={model.ModelId}
              hover
              onClick={openModel(model)}
              sx={{ cursor: "pointer" }}
            >
              <TableCell>{model.ModelName}</TableCell>
              <TableCell>{model.CubeType}</TableCell>
              <TableCell>{model.EpochCount}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}

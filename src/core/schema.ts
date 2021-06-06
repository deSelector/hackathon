export interface Schema {
  cols: Column[];
}
export interface Column {
  id: string;
  name: string;
  col_type: ColumnType;
  size?: number;
  precision?: number;
  hidden?: boolean;
  highlight?: boolean;
  suppress_zero?: boolean;
}

export enum ColumnType {
  Default = 0,
  String,
  Number,
  Date,
  DateTime,
  Timestamp,
  Sparkline
}

export const NUM_SIZE = 8;
export function calcDataWidth(schema: Schema): number {
  return schema?.cols.reduce(
    (p, c) =>
      c.col_type === ColumnType.String || c.col_type === ColumnType.Sparkline
        ? (p += c.size ?? NUM_SIZE)
        : (p += NUM_SIZE),
    0
  );
}

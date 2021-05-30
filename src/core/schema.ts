export interface Schema {
    cols: Column[];
}
export interface Column {
    id: number;
    name: string;
    col_type: ColumnType;
    data_offset: number;
    data_len?: number;
    precision?: number;
    hidden?: boolean;
}

export enum ColumnType {
    Default = 0,
    String,
    Number,
    Date,
    DateTime,
    Timestamp,
}

export const NUM_SIZE = 8;
export function calcDataWidth(schema: Schema): number {
    return schema?.cols.reduce(
        (p, c) => (p += c.data_len ?? NUM_SIZE),
        0
    );
}
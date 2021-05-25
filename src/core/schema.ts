export interface Schema {
    cols: Column[];
}
export interface Column {
    id: number;
    name: string;
    col_type: ColumnType;
    data_offset: number;
}


export enum ColumnType {
    Default = 0,
    String,
    Number,
    Date,
    DateTime,
    Timestamp,
}
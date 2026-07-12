import { createElement } from "react";
import type { JSX, ReactNode } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export interface ListItem {
  id: string;
  label: string;
  disabled?: boolean;
}

export interface ListProps {
  items: ListItem[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function List(props: ListProps): JSX.Element {
  return createElement("List", props);
}

export interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
  expanded?: boolean;
}

export interface TreeProps {
  nodes: TreeNode[];
  selectedId?: string;
  onSelect?: (id: string) => void;
  onToggle?: (id: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Tree({ nodes, selectedId, style }: TreeProps): JSX.Element {
  const renderNode = (node: TreeNode, depth = 0) => {
    const isSelected = node.id === selectedId;
    const hasChildren = node.children && node.children.length > 0;
    const icon = hasChildren ? (node.expanded ? "v " : "> ") : "  ";
    const indent = "  ".repeat(depth);

    return (
      <Flex key={node.id} flexDirection="column">
        <Flex flexDirection="row">
          <Text dim>{indent}</Text>
          <Text dim={!isSelected} bold={isSelected}>
            {icon}
          </Text>
          <Text style={{ inverse: isSelected }}>{node.label}</Text>
        </Flex>
        {node.expanded &&
          hasChildren &&
          node.children?.map((child) => renderNode(child, depth + 1))}
      </Flex>
    );
  };

  return (
    <Flex flexDirection="column" style={style}>
      {nodes.map((node) => renderNode(node, 0))}
    </Flex>
  );
}

export interface TableColumn<T = Record<string, unknown>> {
  key: string;
  header: string;
  width?: number;
  align?: "left" | "center" | "right";
  render?: (value: unknown, row: T) => ReactNode;
}

export interface TableProps<T = Record<string, unknown>> {
  columns: (string | TableColumn<T>)[];
  data?: T[];
  rows?: (string | number | boolean)[][];
  selectedId?: string;
  onSelect?: (id: string) => void;
  style?: Record<string, unknown> | undefined;
}

export function Table({ columns, data, rows, style }: TableProps): JSX.Element {
  // Normalize columns
  const normalizedColumns = columns.map((c, i) => {
    if (typeof c === "string")
      return { key: String(i), header: c, width: Math.max(10, c.length + 2) };
    return { ...c, width: c.width ?? 12 };
  });

  // Normalize rows
  let normalizedRows: (string | number | boolean)[][] = [];
  if (rows) {
    normalizedRows = rows;
  } else if (data) {
    normalizedRows = data.map((item) =>
      normalizedColumns.map((col) => {
        const val = item[col.key];
        return val !== undefined ? val : "";
      }),
    );
  }

  return (
    <Flex flexDirection="column" style={style}>
      {/* Header */}
      <Flex flexDirection="row">
        {normalizedColumns.map((col) => (
          <Box key={`h-${col.key}`} width={col.width}>
            <Text>{col.header}</Text>
          </Box>
        ))}
      </Flex>
      {/* Separator */}
      <Flex flexDirection="row">
        {normalizedColumns.map((col) => (
          <Box key={`s-${col.key}`} width={col.width}>
            <Text dim>{"─".repeat((col.width ?? 10) - 1)}</Text>
          </Box>
        ))}
      </Flex>
      {/* Body */}
      {normalizedRows.map((row) => (
        <Flex key={row.join("|")} flexDirection="row">
          {row.map((cell, cIdx) => (
            <Box key={cell} width={normalizedColumns[cIdx]?.width}>
              <Text>{String(cell)}</Text>
            </Box>
          ))}
        </Flex>
      ))}
    </Flex>
  );
}

export interface DataTableProps<T = Record<string, unknown>> {
  columns: TableColumn<T>[];
  data?: T[];
  rows?: T[]; // Example uses rows instead of data
  sortable?: boolean;
  filterable?: boolean;
  selectedIndex?: number;
  onSelect?: (index: number) => void;
  style?: Record<string, unknown> | undefined;
}

export function DataTable({
  columns,
  data,
  rows,
  selectedIndex,
  style,
}: DataTableProps): JSX.Element {
  const actualData = rows ?? data ?? [];
  return (
    <Flex flexDirection="column" style={style}>
      {/* Header */}
      <Flex flexDirection="row">
        {columns.map((col) => (
          <Box key={`h-${col.key}`} width={col.width ?? 12}>
            <Text>{col.header}</Text>
          </Box>
        ))}
      </Flex>
      {/* Separator */}
      <Flex flexDirection="row">
        {columns.map((col) => (
          <Box key={`s-${col.key}`} width={col.width ?? 12}>
            <Text dim>{"─".repeat((col.width ?? 12) - 1)}</Text>
          </Box>
        ))}
      </Flex>
      {/* Body */}
      {actualData.map((row) => {
        const isSelected = actualData.indexOf(row) === selectedIndex;
        return (
          <Flex key={JSON.stringify(row)} flexDirection="row">
            {columns.map((col) => {
              const val = String(row[col.key] ?? "");
              return (
                <Box key={`c-${col.key}`} width={col.width ?? 12}>
                  <Text style={{ inverse: isSelected }}>{val}</Text>
                </Box>
              );
            })}
          </Flex>
        );
      })}
    </Flex>
  );
}

import type {
  ListItem as CoreListItem,
  TableColumn as CoreTableColumn,
  TreeNode as CoreTreeNode,
  DataTableOptions,
  ListOptions,
  TableOptions,
  TreeOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX } from "react";
import { Box, Flex } from "./layout";
import { Text } from "./typography";

export type ListItem = CoreListItem;

export interface ListProps extends ListOptions {
  style?: Record<string, unknown> | undefined;
}

export function List(props: ListProps): JSX.Element {
  return createElement("List", props);
}

export type TreeNode = CoreTreeNode;

export interface TreeProps extends TreeOptions {
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

export type TableColumn<T = Record<string, unknown>> = CoreTableColumn<T>;

export interface TableProps<T = Record<string, unknown>> extends TableOptions<T> {
  style?: Record<string, unknown> | undefined;
}

export function Table<T extends Record<string, unknown> = Record<string, unknown>>({
  columns,
  data,
  rows,
  style,
}: TableProps<T>): JSX.Element {
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
        const val = item[col.key as keyof T];
        if (typeof val === "string" || typeof val === "number" || typeof val === "boolean") {
          return val;
        }
        return val !== undefined && val !== null ? String(val) : "";
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
      {normalizedRows.map((row, rIdx) => {
        return (
          // biome-ignore lint/suspicious/noArrayIndexKey: Table rows lack unique IDs
          <Flex key={`row-${rIdx}`} flexDirection="row">
            {row.map((cell, cIdx) => {
              return (
                // biome-ignore lint/suspicious/noArrayIndexKey: Table cells lack unique IDs
                <Box key={`cell-${cIdx}`} width={normalizedColumns[cIdx]?.width}>
                  <Text>{String(cell)}</Text>
                </Box>
              );
            })}
          </Flex>
        );
      })}
    </Flex>
  );
}

export interface DataTableProps<T = Record<string, unknown>> extends DataTableOptions<T> {
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

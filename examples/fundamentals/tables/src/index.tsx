import { CommandBuffer, createReconciler } from "@bettertui/core";
import {
  Badge,
  DataTable,
  Flex,
  Heading,
  Provider,
  Separator,
  Spacer,
  StatusLine,
  Table,
  Text,
} from "@bettertui/react";

const buffer = new CommandBuffer();
const reconciler = createReconciler(buffer);

const employees = [
  {
    name: "Alice Johnson",
    role: "Admin",
    department: "Engineering",
    salary: 120000,
    status: "active",
  },
  { name: "Bob Smith", role: "Editor", department: "Marketing", salary: 85000, status: "active" },
  { name: "Charlie Brown", role: "Viewer", department: "Sales", salary: 65000, status: "inactive" },
  {
    name: "Diana Prince",
    role: "Editor",
    department: "Engineering",
    salary: 110000,
    status: "active",
  },
  { name: "Eve Davis", role: "Admin", department: "HR", salary: 95000, status: "active" },
  {
    name: "Frank Miller",
    role: "Viewer",
    department: "Marketing",
    salary: 70000,
    status: "active",
  },
  {
    name: "Grace Lee",
    role: "Editor",
    department: "Engineering",
    salary: 115000,
    status: "active",
  },
  { name: "Henry Wilson", role: "Viewer", department: "Sales", salary: 60000, status: "inactive" },
];

const columns = [
  { key: "name", header: "Name", width: 16 },
  { key: "role", header: "Role", width: 10 },
  { key: "department", header: "Department", width: 14 },
  { key: "salary", header: "Salary", width: 10, align: "right" as const },
  { key: "status", header: "Status", width: 10 },
];

type SortColumn = "name" | "role" | "department" | "salary" | "status";

function sortEmployees(
  data: typeof employees,
  column: SortColumn,
  direction: "asc" | "desc",
): typeof employees {
  const sorted = [...data].sort((a, b) => {
    const av = a[column];
    const bv = b[column];
    if (typeof av === "number" && typeof bv === "number") {
      return direction === "asc" ? av - bv : bv - av;
    }
    return direction === "asc"
      ? String(av).localeCompare(String(bv))
      : String(bv).localeCompare(String(av));
  });
  return sorted;
}

function departmentSummary(data: typeof employees) {
  const groups: Record<string, { count: number; totalSalary: number; active: number }> = {};
  for (const e of data) {
    if (!groups[e.department]) {
      groups[e.department] = { count: 0, totalSalary: 0, active: 0 };
    }
    groups[e.department].count++;
    groups[e.department].totalSalary += e.salary;
    if (e.status === "active") groups[e.department].active++;
  }
  return Object.entries(groups).map(([dept, info]) => ({
    Department: dept,
    Count: String(info.count),
    "Avg Salary": `$${Math.round(info.totalSalary / info.count / 1000)}k`,
    Active: `${info.active}/${info.count}`,
  }));
}

const sortLabels: Record<SortColumn, string> = {
  name: "1:Name",
  role: "2:Role",
  department: "3:Dept",
  salary: "4:Salary",
  status: "5:Status",
};

interface TablesAppProps {
  selectedIndex: number;
  sortColumn: SortColumn;
  sortDirection: "asc" | "desc";
}

function TablesApp({ selectedIndex, sortColumn, sortDirection }: TablesAppProps) {
  const sorted = sortEmployees(employees, sortColumn, sortDirection);
  const summaryRows = departmentSummary(employees);
  const arrow = sortDirection === "asc" ? " ^" : " v";

  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Flex flexDirection="row" alignItems="center">
          <Heading level={2}>Table Examples</Heading>
          <Spacer />
          <Badge variant="info">{employees.length} rows</Badge>
        </Flex>

        <Separator />

        <Heading level={3}>Basic Table</Heading>
        <Table
          columns={["Name", "Role", "Status"]}
          rows={employees.map((e) => [e.name, e.role, e.status])}
        />

        <Separator />

        <Heading level={3}>
          Data Table with Selection
          <Text dimColor>
            {" "}
            (sorted by {sortColumn}
            {arrow})
          </Text>
        </Heading>
        <DataTable columns={columns} rows={sorted} selectedIndex={selectedIndex} />

        <Separator />

        <Heading level={3}>Department Summary</Heading>
        <Table
          columns={["Department", "Count", "Avg Salary", "Active"]}
          rows={summaryRows.map((r) => [r.Department, r.Count, r["Avg Salary"], r.Active])}
        />

        <Separator />

        <StatusLine
          items={[
            { label: "Selected", value: sorted[selectedIndex]?.name ?? "-" },
            { label: "Row", value: `${selectedIndex + 1}/${sorted.length}` },
            { label: "Sort", value: sortLabels[sortColumn] },
            { separator: true },
            { label: "j/k", value: "nav" },
            { label: "1-5", value: "sort" },
            { label: "q", value: "quit" },
          ]}
        />
      </Flex>
    </Provider>
  );
}

let selectedIndex = 0;
let sortColumn: SortColumn = "name";
const sortDirection: "asc" | "desc" = "asc";

function renderApp() {
  const element = (
    <TablesApp
      selectedIndex={selectedIndex}
      sortColumn={sortColumn}
      sortDirection={sortDirection}
    />
  );
  reconciler.createInstance("Provider", { children: element });
}

console.log("BetterTUI Tables Demo");
console.log("Navigate with j/k, sort with 1-5, q to quit");

renderApp();

process.stdin.setRawMode?.(true);
process.stdin.resume();
process.stdin.on("data", (data) => {
  const key = data.toString();
  const sorted = sortEmployees(employees, sortColumn, sortDirection);

  if (key === "j" || key === "\x1b[B") {
    selectedIndex = Math.min(selectedIndex + 1, sorted.length - 1);
    renderApp();
  } else if (key === "k" || key === "\x1b[A") {
    selectedIndex = Math.max(selectedIndex - 1, 0);
    renderApp();
  } else if (key === "1") {
    sortColumn = "name";
    selectedIndex = 0;
    renderApp();
  } else if (key === "2") {
    sortColumn = "role";
    selectedIndex = 0;
    renderApp();
  } else if (key === "3") {
    sortColumn = "department";
    selectedIndex = 0;
    renderApp();
  } else if (key === "4") {
    sortColumn = "salary";
    selectedIndex = 0;
    renderApp();
  } else if (key === "5") {
    sortColumn = "status";
    selectedIndex = 0;
    renderApp();
  } else if (key === "q") {
    process.exit(0);
  }
});

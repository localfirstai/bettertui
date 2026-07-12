import { type BenchmarkMetrics, sampleReport } from "@lib/bench";
import { useMemo, useState } from "react";

type MetricKey = keyof Pick<
  BenchmarkMetrics,
  | "layoutMs"
  | "renderMs"
  | "frameGenerateMs"
  | "fps"
  | "memoryRssMb"
  | "startupMs"
  | "inputLatencyMs"
  | "bundleKb"
>;

interface MetricConfig {
  key: MetricKey;
  label: string;
  unit: string;
  lowerIsBetter: boolean;
}

const metrics: MetricConfig[] = [
  { key: "frameGenerateMs", label: "Frame Generation", unit: "ms", lowerIsBetter: true },
  { key: "layoutMs", label: "Layout Pass", unit: "ms", lowerIsBetter: true },
  { key: "renderMs", label: "Render Pass", unit: "ms", lowerIsBetter: true },
  { key: "fps", label: "Frames Per Second", unit: "fps", lowerIsBetter: false },
  { key: "memoryRssMb", label: "Memory (RSS)", unit: "MB", lowerIsBetter: true },
  { key: "startupMs", label: "Startup Time", unit: "ms", lowerIsBetter: true },
  { key: "inputLatencyMs", label: "Input Latency", unit: "ms", lowerIsBetter: true },
  { key: "bundleKb", label: "Bundle Size", unit: "KB", lowerIsBetter: true },
];

export function SideBySide() {
  const [selectedMetric, setSelectedMetric] = useState<MetricKey>("frameGenerateMs");
  const [selectedApp, setSelectedApp] = useState<string>("stress-test");

  const data = useMemo(() => {
    const ot = sampleReport.opentui.find((m) => m.app === selectedApp);
    const bt = sampleReport.bettertui.find((m) => m.app === selectedApp);
    return { ot, bt };
  }, [selectedApp]);

  const config = metrics.find((m) => m.key === selectedMetric) ?? metrics[0];
  const otVal = data.ot?.[selectedMetric] ?? 0;
  const btVal = data.bt?.[selectedMetric] ?? 0;
  const maxVal = Math.max(otVal, btVal, 0.001);
  const otPct = (otVal / maxVal) * 100;
  const btPct = (btVal / maxVal) * 100;
  const improvement = otVal > 0 ? Math.abs((1 - btVal / otVal) * 100) : 0;
  const better = config.lowerIsBetter ? btVal < otVal : btVal > otVal;

  const appOptions = sampleReport.opentui.map((m) => m.app);

  return (
    <div className="mx-auto max-w-3xl">
      <div className="rounded-xl border border-border bg-card/50 p-6 backdrop-blur-sm md:p-8">
        {/* Controls */}
        <div className="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
          <div className="flex flex-wrap gap-2">
            {metrics.map((m) => (
              <button
                type="button"
                key={m.key}
                onClick={() => setSelectedMetric(m.key)}
                className={`rounded-md px-2.5 py-1 text-xs font-medium transition-colors ${
                  selectedMetric === m.key
                    ? "bg-terminal-muted text-terminal"
                    : "bg-muted text-muted-foreground hover:bg-muted/70 hover:text-foreground"
                }`}
              >
                {m.label}
              </button>
            ))}
          </div>
        </div>

        <div className="mb-6">
          <label
            htmlFor="app-select"
            className="mb-2 block text-xs font-medium uppercase tracking-wider text-muted-foreground"
          >
            Benchmark App
          </label>
          <select
            id="app-select"
            value={selectedApp}
            onChange={(e) => setSelectedApp(e.target.value)}
            className="w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
          >
            {appOptions.map((app) => (
              <option key={app} value={app}>
                {app.replace(/-/g, " ").replace(/\b\w/g, (c) => c.toUpperCase())}
              </option>
            ))}
          </select>
        </div>

        {/* Comparison bars */}
        <div className="space-y-5">
          {/* OpenTUI */}
          <div>
            <div className="mb-1.5 flex items-center justify-between">
              <span className="text-sm text-muted-foreground">OpenTUI</span>
              <span className="font-mono text-sm text-muted-foreground">
                {otVal.toFixed(2)} {config.unit}
              </span>
            </div>
            <div className="relative h-6 overflow-hidden rounded-md bg-muted-foreground/10">
              <div
                className="h-full rounded-md bg-muted-foreground/40 transition-all duration-500"
                style={{ width: `${otPct}%` }}
              />
            </div>
          </div>

          {/* BetterTUI */}
          <div>
            <div className="mb-1.5 flex items-center justify-between">
              <span className="text-sm font-medium text-terminal">BetterTUI</span>
              <span className="font-mono text-sm font-semibold text-terminal">
                {btVal.toFixed(2)} {config.unit}
              </span>
            </div>
            <div className="relative h-6 overflow-hidden rounded-md bg-terminal-muted/10">
              <div
                className="h-full rounded-md bg-terminal transition-all duration-500"
                style={{ width: `${btPct}%` }}
              />
            </div>
          </div>
        </div>

        {/* Result badge */}
        <div className="mt-6 flex items-center justify-center gap-3 rounded-lg bg-muted/40 px-4 py-3">
          {better ? (
            <>
              <svg
                className="h-5 w-5 text-terminal"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                strokeWidth="3"
                role="img"
                aria-label="Improvement"
              >
                <path strokeLinecap="round" strokeLinejoin="round" d="M2 17L14 5M14 5H7M14 5v7" />
              </svg>
              <span className="text-sm font-medium text-terminal">
                BetterTUI is <strong>{improvement.toFixed(0)}%</strong>{" "}
                {config.lowerIsBetter ? "faster" : "higher"}
              </span>
            </>
          ) : (
            <span className="text-sm text-muted-foreground">Comparable performance</span>
          )}
        </div>
      </div>
    </div>
  );
}

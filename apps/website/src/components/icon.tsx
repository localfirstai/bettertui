import type { SVGProps } from "react";

import { AstroIconDark } from "./ui/svgs/astro-icon-dark";
import { AstroIconLight } from "./ui/svgs/astro-icon-light";
import { AstroWordmarkDark } from "./ui/svgs/astro-wordmark-dark";
import { AstroWordmarkLight } from "./ui/svgs/astro-wordmark-light";
import { Preact } from "./ui/svgs/preact";
import { ReactDark } from "./ui/svgs/react-dark";
import { ReactLight } from "./ui/svgs/react-light";
import { ReactWordmarkDark } from "./ui/svgs/react-wordmark-dark";
import { ReactWordmarkLight } from "./ui/svgs/react-wordmark-light";
import { Solidjs } from "./ui/svgs/solidjs";
import { Svelte } from "./ui/svgs/svelte";
import { Vue } from "./ui/svgs/vue";

type IconProps = SVGProps<SVGSVGElement> & { class?: string };

export {
  AstroIconDark,
  AstroIconLight,
  AstroWordmarkDark,
  AstroWordmarkLight,
  Preact,
  ReactDark,
  ReactLight,
  ReactWordmarkDark,
  ReactWordmarkLight,
  Solidjs,
  Svelte,
  Vue,
};

// GitHub brand mark (used in navbar, footer, hero).
export function GithubIcon(props: IconProps) {
  return (
    <svg {...props} viewBox="0 0 1024 1024" fill="currentColor" aria-hidden="true">
      <path
        fill="currentColor"
        fill-rule="evenodd"
        d="M512 0C229.12 0 0 229.12 0 512c0 226.56 146.56 417.92 350.08 485.76 25.6 4.48 35.2-10.88 35.2-24.32 0-12.16-.64-52.48-.64-95.36-128.64 23.68-161.92-31.36-172.16-60.16-5.76-14.72-30.72-60.16-52.48-72.32-17.92-9.6-43.52-33.28-.64-33.92 40.32-.64 69.12 37.12 78.72 52.48 46.08 77.44 119.68 55.68 149.12 42.24 4.48-33.28 17.92-55.68 32.64-68.48-113.92-12.8-232.96-56.96-232.96-252.8 0-55.68 19.84-101.76 52.48-137.6-5.12-12.8-23.04-65.28 5.12-135.68 0 0 42.88-13.44 140.8 52.48 40.96-11.52 84.48-17.28 128-17.28s87.04 5.76 128 17.28c97.92-66.56 140.8-52.48 140.8-52.48 28.16 70.4 10.24 122.88 5.12 135.68 32.64 35.84 52.48 81.28 52.48 137.6 0 196.48-119.68 240-233.6 252.8 18.56 16 34.56 46.72 34.56 94.72 0 68.48-.64 123.52-.64 140.8 0 13.44 9.6 29.44 35.2 24.32C877.44 929.92 1024 737.92 1024 512 1024 229.12 794.88 0 512 0"
        clip-rule="evenodd"
      />
    </svg>
  );
}

// BetterTUI terminal/box brand mark (used in navbar + footer logotype).
export function LogoIcon(props: IconProps) {
  return (
    <svg
      {...props}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      aria-hidden="true"
    >
      <rect x="2" y="3" width="20" height="18" rx="2" />
      <polyline points="8 10 11 13 8 16" />
      <line x1="14" y1="16" x2="16" y2="16" />
    </svg>
  );
}

// Hamburger / menu toggle icon (used in the mobile navbar).
export function MenuIcon(props: IconProps) {
  return (
    <svg
      {...props}
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      stroke-width="2"
      aria-hidden="true"
    >
      <line x1="3" y1="6" x2="21" y2="6" />
      <line x1="3" y1="12" x2="21" y2="12" />
      <line x1="3" y1="18" x2="21" y2="18" />
    </svg>
  );
}

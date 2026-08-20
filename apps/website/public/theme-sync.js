/**
 * Keeps the landing page and Starlight docs on the same light/dark theme.
 *
 * The landing page stores its preference under `localStorage.theme` and drives
 * a `dark` class; Starlight stores `localStorage.starlight-theme` and drives a
 * `data-theme` attribute. This script reconciles both sources so toggling the
 * theme anywhere stays in sync everywhere.
 */
(() => {
  const root = document.documentElement;

  const read = (key) => {
    try {
      return localStorage.getItem(key);
    } catch {
      return null;
    }
  };

  const write = (key, value) => {
    try {
      localStorage.setItem(key, value);
    } catch {
      /* storage unavailable — theme still applies for this session */
    }
  };

  const systemTheme = () =>
    window.matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark";

  const domTheme = () => {
    const data = root.getAttribute("data-theme");
    if (data === "dark" || data === "light") return data;
    return root.classList.contains("dark") ? "dark" : "light";
  };

  const storedTheme = () => {
    const theme = read("theme") ?? read("starlight-theme");
    return theme === "dark" || theme === "light" ? theme : systemTheme();
  };

  const apply = (theme) => {
    if (root.getAttribute("data-theme") !== theme) root.setAttribute("data-theme", theme);
    root.classList.toggle("dark", theme === "dark");
    write("theme", theme);
    write("starlight-theme", theme);
    if (window.StarlightThemeProvider) window.StarlightThemeProvider.updatePickers(theme);
  };

  apply(storedTheme());

  new MutationObserver(() => apply(domTheme())).observe(root, {
    attributes: true,
    attributeFilter: ["data-theme", "class"],
  });
})();

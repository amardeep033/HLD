# 07 · Frontend — what runs in the user's browser/device

> skin. the part the human touches.
> prev ← [06](06_cross_cutting.md) · next → [08 security](08_security_identity.md)

## map

```
frontend
├── app type (what gets shipped)
│   ├── website / MPA ── every click = new html page from server
│   ├── SPA ── one html, js swaps views, backend returns only data (json)
│   ├── PWA ── spa + service worker + manifest → installable, offline
│   ├── micro-frontends / microapps ── one screen stitched from pieces owned by diff teams
│   ├── mobile ── native (swift/kotlin) | cross-platform (React Native, Flutter) | hybrid webview (Ionic, Capacitor)
│   └── desktop ── native | Electron | Tauri
├── rendering (where + when html is built)
│   ├── CSR | SSR | SSG | ISR | streaming SSR
│   └── hydration family ── full hydration | partial / islands (Astro) | resumability (Qwik) | React Server Components
├── micro-frontend integration (siblings)
│   └── build-time npm pkg | runtime module federation | single-spa | iframe | web components | server/edge composition
├── ui architecture ── MVC | MVP | MVVM | Flux/Redux | MVU → 09
├── state
│   ├── local component state
│   ├── global store ── Redux | Zustand | MobX | NgRx | Pinia | Context
│   ├── server state / data fetching ── React Query (TanStack) | SWR | Apollo | RTK Query
│   └── url state ── route + query params
├── design → code
│   ├── design tool ── Figma | Sketch | Adobe XD | Penpot
│   ├── design tokens ── color / spacing / typography as named variables
│   ├── design system ── Fluent UI (MS) | Material (Google) | Carbon (IBM) | Polaris (Shopify) | Ant | Chakra
│   ├── component lib flavour ── styled (MUI, Fluent) | headless (Radix, Headless UI) | copy-paste (shadcn/ui)
│   └── wrapper lib ── company's own components over Fluent/Material → apps never import vendor directly, rebrand/swap in one place
├── repo strategy ── polyrepo | monorepo (Nx, Turborepo, pnpm/yarn workspaces, Lerna, Rush)
├── frameworks ── React | Angular | Vue | Svelte | Solid
│   └── meta-frameworks ── Next (react) | Remix / React Router | Nuxt (vue) | SvelteKit | Astro | Angular SSR
├── build toolchain ── bundler (webpack, vite, esbuild, rollup, turbopack, rspack) | transpile (babel, swc, tsc) | lint (eslint) | format (prettier)
├── styling ── plain CSS | SASS | CSS modules | CSS-in-JS (styled-components, emotion, griffel) | utility (Tailwind)
├── browser storage ── cookie | localStorage | sessionStorage | IndexedDB | Cache API (service worker)
├── talking to backend ── fetch/axios → REST | GraphQL client | ws/sse → 04 · via BFF → 09
├── auth in browser ── cookies, tokens, OAuth/OIDC → 08
└── performance ── core web vitals (LCP, INP, CLS) | code splitting | lazy load | tree shaking | image opt | prefetch
```

## rendering (siblings)

| | html built where | when | first paint | SEO | one-liner |
|---|---|---|---|---|---|
| CSR | browser | on each visit | slow (blank → js → render) | weak | html/css/js downloaded once & kept by client; after that backend returns data not views |
| SSR | server | per request | fast | ✓ | server renders full html each request, then js hydrates |
| SSG | build machine | at build time | fastest (CDN) | ✓ | pre-built static html, rebuild to change |
| ISR | server | build + re-gen after N sec | fast | ✓ | SSG that refreshes pages in background |
| streaming SSR | server | per request, in chunks | fast | ✓ | send shell first, stream slow parts later |
| server MVC (old SSR) | server | per request | fast | ✓ | jsp / razor / thymeleaf, full page reload per click |

- hydration: attach js event handlers to server-made html so it becomes interactive.

## app types (siblings)

| | pages | backend returns | one-liner |
|---|---|---|---|
| MPA | many html | html | classic website |
| SPA | one html | json | app-like, client routing |
| PWA | one html + SW | json | spa that installs + works offline |
| micro-frontend | shell + remote pieces | json | team autonomy at UI level (like microservices for UI) |

## browser storage (siblings)

| | size | sent to server auto? | JS can read? | lifetime |
|---|---|---|---|---|
| cookie | ~4 KB | ✓ every request to domain | ✗ if HttpOnly | expiry / session |
| localStorage | ~5–10 MB | ✗ | ✓ | forever |
| sessionStorage | ~5 MB | ✗ | ✓ | tab closes |
| IndexedDB | large | ✗ | ✓ (async) | forever |
| Cache API | large | ✗ | ✓ (SW) | forever |

## monorepo vs polyrepo

| | monorepo | polyrepo |
|---|---|---|
| code | all apps + libs in one repo | one repo per app/lib |
| sharing | import directly | publish npm pkg + version |
| tooling | needs Nx/Turbo (affected builds, cache) | simple per repo |
| pain | big repo, CI smarts needed | version drift, dependency hell |

## figma → production chain

Figma design → design tokens → design system (Fluent UI) → company wrapper lib → app components → screens

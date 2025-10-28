---
trigger: always_on
---

# Svelte 5 & SvelteKit Quick Reference (For Windsurf)

## Svelte 5 Core (USE SVELTE 5 UNLESS TOLD OTHERWISE)

### Runes (keywords, NOT functions)
- `$state(val)` - reactive state: `let count = $state(0)`
- `$derived(expr)` - computed: `const double = $derived(count * 2)`
- `$derived.by(() => {...})` - complex derivations
- `$effect(() => {...})` - side effects after DOM update
- `$effect.pre(() => {...})` - before DOM update
- `$props()` - component props: `let { foo = true, bar } = $props()`
- `$bindable()` - two-way props: `let { value = $bindable() } = $props()`

### Key Changes from Svelte 4
- Events: `<button onclick={...}>` not `on:click`
- Reactive: `$state()` not `let`; `$derived()` not `$:`; `$effect()` not `$:` side effects
- Props: `$props()` not `export let`

### State
- Deep reactivity on objects/arrays
- Don't destructure proxies
- `$state.raw()` for shallow state
- `$state.snapshot()` for plain copies

### Snippets (replaces slots)
```svelte
{#snippet name(param1, param2)}
  <p>{param1}</p>
{/snippet}
{@render name(arg1, arg2)}
```
Pass as props: `<Comp {snippetName} />`
Default children: `let { children } = $props(); {@render children()}`

### Async (experimental, needs `experimental.async: true`)
```svelte
<svelte:boundary>
  <p>{await getData()}</p>
  {#snippet pending()}<p>loading...</p>{/snippet}
</svelte:boundary>
```

### Other
- `<svelte:boundary>` - error boundaries with `{#snippet failed(error, reset)}`
- `class={{ cool, lame: !cool }}` - conditional classes

## SvelteKit Essentials

### Project Setup
```bash
npx sv create
```

package.json devDeps: `@sveltejs/kit`, `@sveltejs/adapter-auto`, `@sveltejs/vite-plugin-svelte`, `svelte`, `vite`

### File Structure
- `src/routes/` - filesystem router (`+page.svelte` = route)
- `src/lib/` - shared code (`$lib` alias)
- `src/lib/server/` - server-only (`$lib/server`)
- `+page.svelte` - UI component
- `+page.js` - universal load (client+server)
- `+page.server.js` - server-only load + actions
- `+layout.svelte` - shared UI with `{@render children()}`
- `+layout(.server).js` - layout data
- `+server.js` - API endpoints (GET, POST, etc.)
- `+error.svelte` - error page

### Routing
- `[param]` - dynamic segment
- `[...rest]` - rest params
- `[[optional]]` - optional
- `(group)` - layout groups (no URL impact)
- `+page@.svelte` - break layout chain

### Loading Data
```js
// +page.js
export async function load({ fetch, params }) {
  const res = await fetch(`/api/${params.id}`);
  return { data: await res.json() };
}
```
Access in component: `let { data } = $props()`

Server load: `+page.server.js` for DB/secrets, returns serializable data

### Forms & Actions
```js
// +page.server.js
export const actions = {
  default: async ({ request }) => {
    const data = await request.formData();
    // process...
    return fail(400, { error: 'msg' }); // or redirect(303, '/success')
  }
};
```
```svelte
<script>
  import { enhance } from '$app/forms';
  let { form } = $props();
</script>
<form method="POST" use:enhance>
  <input name="field" value={form?.field ?? ''}>
  {#if form?.error}<p>{form.error}</p>{/if}
</form>
```

### Remote Functions (experimental, needs `experimental.remoteFunctions: true`)
```js
// data.remote.js
import { query, form, command } from '$app/server';

export const getPosts = query(async () => { /* ... */ });
export const createPost = form(async (data) => { /* ... */ });
export const addLike = command(schema, async (id) => { /* ... */ });
```
Use: `{#each await getPosts() as post}` or `<form {...createPost}>`

### Page Options
- `export const prerender = true` - SSG
- `export const ssr = false` - SPA mode
- `export const csr = false` - no hydration

### Key Imports
```js
import { error, fail, redirect, json } from '@sveltejs/kit';
import { goto, invalidate } from '$app/navigation';
import { page } from '$app/state';
import { env } from '$env/dynamic/private'; // server-only
import { PUBLIC_KEY } from '$env/static/public';
```

### Server-Only
- `$env/static/private`, `$env/dynamic/private`
- `$app/server`
- `*.server.js` or `src/lib/server/*`

### Hooks
```js
// hooks.server.js
export async function handle({ event, resolve }) {
  event.locals.user = await getUser(event.cookies);
  return resolve(event);
}
```

### Adapters
`svelte.config.js`:
```js
import adapter from '@sveltejs/adapter-auto';
export default { kit: { adapter: adapter() } };
```

### Navigation
- `goto('/path')` - programmatic
- `invalidate(url)` - rerun loads
- `preloadData('/path')` - prefetch
- `pushState('', { state })` - shallow routing

### Images
Use `@sveltejs/enhanced-img`:
```svelte
<enhanced:img src="./photo.jpg" alt="..." />
```

That's the core. Refer to docs for advanced patterns.
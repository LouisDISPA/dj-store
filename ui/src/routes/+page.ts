import { env } from '$lib/utils';

// Disable SSR for index page because of a bug with base_href
// TODO : Fix this bug and re-enable prerendering
export const ssr = env.BASE_HREF === '';
export const prerender = env.BASE_HREF === '';

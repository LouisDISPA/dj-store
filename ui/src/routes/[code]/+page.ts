import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
	await new Promise((resolve) => setTimeout(resolve, 2000));
	return { code: params.code };
};

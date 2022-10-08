export async function load({ params }: { params: { code: string } }) {
	console.log(params);
	await new Promise((resolve) => setTimeout(resolve, 1000));
	return { code: params.code };
}

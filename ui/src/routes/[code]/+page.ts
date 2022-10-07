
export function load({ params }: { params: { code: string } }) {
    console.log(params);
    return { code: params.code };
}
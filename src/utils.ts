// eslint-disable-next-line @typescript-eslint/no-unused-vars
export function assertUnreachable(_x: never): never {
    throw new Error("Didn't expect to get here");
}
export function toMb(value: bigint): number {
    return (Number(value / 1024n) / 1024.0);
}
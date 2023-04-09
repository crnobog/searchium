declare global {
    interface Array<T> {
        last: () => T | undefined;
    }
}

Array.prototype.last = function (): unknown {
    return this.length > 0 ? this[this.length - 1] : undefined;
};

export { };
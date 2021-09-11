/**
 * Custom iterator type inspired by rust's Iterator.
 */
export class PeekableIterator<T> implements IterableIterator<T> {
    #iter: IterableIterator<T>;
    #peek: IteratorResult<T>;

    constructor(iter: IterableIterator<T>) {
        this.#iter = iter;
        this.#peek = iter.next();
    }
    [Symbol.iterator](): PeekableIterator<T> {
        return this;
    }
    next(): IteratorResult<T> {
        const next = this.#peek;
        if (!this.#peek.done)
            this.#peek = this.#iter.next();
        return next;
    }
    peek(): IteratorResult<T> {
        return this.#peek;
    }
    skip(check: (arg: T) => boolean): PeekableIterator<T> {
        function* skip(iter: PeekableIterator<T>, check: (arg: T) => boolean) {
            let e = iter.next();
            for (; !e.done && check(e.value); e = iter.next());
            if (!e.done) {
                yield e.value;
                yield* iter;
            }
        }
        return new PeekableIterator(skip(this, check));
    }
    take(check: (arg: T) => boolean): PeekableIterator<T> {
        function* take(iter: PeekableIterator<T>, check: (arg: T) => boolean) {
            for (const elem of iter) {
                if (!check(elem)) return;
                yield elem;
            }
        }
        return new PeekableIterator(take(this, check));
    }
    filter(check: (arg: T) => boolean): PeekableIterator<T> {
        function* filter(iter: PeekableIterator<T>, check: (arg: T) => boolean) {
            for (const elem of iter)
                if (check(elem)) yield elem;
        }
        return new PeekableIterator(filter(this, check));
    }
    map<U>(op: (arg: T) => U): PeekableIterator<U> {
        function* map<U>(iter: PeekableIterator<T>, op: (arg: T) => U) {
            for (const elem of iter) yield op(elem);
        }
        return new PeekableIterator(map(this, op));
    }
}
export function iter<T>(iter: IterableIterator<T>): PeekableIterator<T> {
    return new PeekableIterator(iter);
}
export function range(begin: number, end: number): PeekableIterator<number> {
    function* range(begin: number, end: number) {
        for (let i = begin; i < end; i++) yield i;
    }
    return new PeekableIterator(range(begin, end));
}

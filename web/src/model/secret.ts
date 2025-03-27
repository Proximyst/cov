interface Stringable {
  toString(): string;
}

export default class Secret<T extends Stringable> {
  constructor(private value: T) {}

  toString(): string {
    return "[secret]";
  }

  expose(): string {
    return this.value.toString();
  }

  update(newValue: T) {
    this.value = newValue;
  }
}

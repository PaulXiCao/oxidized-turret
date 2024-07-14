export function html(strings, ...values) {
  return String.raw({ raw: strings }, ...values);
}

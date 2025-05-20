import { tooltip as svt } from "svooltip";
import { type Options } from "svooltip/types";

export function tooltip(
  node: HTMLElement,
  [content, constant = false]: [string, boolean?]
) {
  const options: Options = {
    content,
    offset: 4,
    shiftPadding: 4,
    constant,
  };
  svt(node, options);
}

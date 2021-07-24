import React, { MouseEventHandler } from "react";
import css from "../../styles/Menu.module.scss"

type Props = { onClick: MouseEventHandler };

export default function BurgerMenu({ onClick }: Props) {
  return (
    <button className={css.Button} onClick={onClick}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M4 6h16M4 12h16M4 18h16"
        />
      </svg>
    </button>
  );
}

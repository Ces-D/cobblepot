import React from "react";

type Props = {
  clickHandler: React.MouseEventHandler;
};

export default function Burger({ clickHandler }: Props) {
  return (
    <button className="rounded-md hover:bg-secondary hover:bg-opacity-50" onClick={clickHandler}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="h-6 w-6"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M4 6h16M4 12h8m-8 6h16"
        />
      </svg>
    </button>
  );
}

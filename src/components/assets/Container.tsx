import React, { ReactElement } from "react";

type Props = { children: React.ReactNode };

export default function Container({ children }: Props) {
  return <div className="min-h-screen bg-light dark:bg-dark">{children}</div>;
}

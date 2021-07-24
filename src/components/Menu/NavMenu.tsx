import React from "react";
import css from "../../styles/Menu.module.scss";

type Props = {
  display: boolean;
};

export default function NavMenu({ display }: Props) {
  return (
    <div className={display ? css.NavMenu : "hidden"}>
      <div className={css.Item}>Hello</div>
      <div className={css.Item}>World</div>
      <div className={css.Item}>Ja</div>
      <div className={css.Item}>ssadasdasd</div>
    </div>
  );
}

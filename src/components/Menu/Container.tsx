import React from "react";
import BurgerMenu from "./BurgerMenu";
import NavMenu from "./NavMenu";
import css from "../../styles/Menu.module.scss";

export default function Container() {
  const [menuDisplay, setMenuDisplay] = React.useState<boolean>();
  const [navDisplay, setNavDisplay] = React.useState(true);

  React.useEffect(() => {
    const windowWidth = window.innerWidth<=420
    setMenuDisplay(windowWidth);
    setNavDisplay(!windowWidth)
  }, []);

  return (
    <div className={css.Container}>
      {menuDisplay ? <BurgerMenu onClick={() => setNavDisplay(!navDisplay)} /> : ""}
      <NavMenu display={navDisplay} />
    </div>
  );
}

import React, { useState, useEffect } from "react";
import Burger from "./Burger";
import LeftNav from "./LeftNav";

export default function Header() {
  const [displayBurger, setDisplayBurger] = useState<boolean>();
  const [navDisplay, setNavDisplay] = useState<boolean>(true);

  useEffect(() => {
    const windowWidth = window.innerWidth <= 420;
    setDisplayBurger(windowWidth);
    setNavDisplay(!windowWidth);
  }, []);

  return (
    <>
      <header className="flex bg-dark dark:bg-light h-20 px-2 md:px-8 items-center justify-between text-secondary dark:text-primary">
        <div className="flex gap-x-3 items-center">
          {displayBurger ? (
            <Burger clickHandler={() => setNavDisplay(!navDisplay)} />
          ) : (
            <></>
          )}
          <h1 className="font-bold text-xl">Bank of You</h1>
        </div>
        <div className="text-xs font-semibold">
          <h2 className="mb-2">Accounting Tools</h2>
          <h3>Name of Current Page</h3>
        </div>
      </header>
      {navDisplay ? <LeftNav /> : <></>}
    </>
  );
}

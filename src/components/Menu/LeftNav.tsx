import React, { useEffect, useState } from "react";

export default function LeftNav() {
  const [displayDescription, setDisplayDescription] = useState<boolean>(true);

  const [navDisplay, setNavDisplay] = useState<boolean>(true);

  useEffect(() => {
    const windowWidth = window.innerWidth <= 420;
    setNavDisplay(!windowWidth);
  }, []);

  const ArrowLeft = (
    <button onClick={() => setDisplayDescription(!displayDescription)}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="h-5 w-5 animate-pulse"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fillRule="evenodd"
          d="M7.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l2.293 2.293a1 1 0 010 1.414z"
          clipRule="evenodd"
        />
      </svg>
    </button>
  );

  const ArrowRight = (
    <button onClick={() => setDisplayDescription(!displayDescription)}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        className="h-5 w-5 animate-pulse"
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fillRule="evenodd"
          d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z"
          clipRule="evenodd"
        />
      </svg>
    </button>
  );

  return (
    <nav className="bg-dark dark:bg-light h-full w-52 sm:w-max fixed">
      <ol className="px-2 list-none list-inside text-primary">
        <div className={navDisplay?"flex justify-end pr-1":"hidden"}>
          {displayDescription ? ArrowLeft : ArrowRight}
        </div>
        <li className="flex gap-x-4 p-2 my-2 border-t-2 dark:hover:bg-dark hover:bg-light hover:bg-opacity-60 rounded">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z" />
          </svg>
          <p className={displayDescription ? "inline-block" : "hidden"}>Summary</p>
        </li>
        <li className="flex gap-x-4 p-2 my-2 border-t-2 dark:hover:bg-dark hover:bg-light hover:bg-opacity-60 rounded">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fillRule="evenodd"
              d="M6 2a2 2 0 00-2 2v12a2 2 0 002 2h8a2 2 0 002-2V4a2 2 0 00-2-2H6zm1 2a1 1 0 000 2h6a1 1 0 100-2H7zm6 7a1 1 0 011 1v3a1 1 0 11-2 0v-3a1 1 0 011-1zm-3 3a1 1 0 100 2h.01a1 1 0 100-2H10zm-4 1a1 1 0 011-1h.01a1 1 0 110 2H7a1 1 0 01-1-1zm1-4a1 1 0 100 2h.01a1 1 0 100-2H7zm2 1a1 1 0 011-1h.01a1 1 0 110 2H10a1 1 0 01-1-1zm4-4a1 1 0 100 2h.01a1 1 0 100-2H13zM9 9a1 1 0 011-1h.01a1 1 0 110 2H10a1 1 0 01-1-1zM7 8a1 1 0 000 2h.01a1 1 0 000-2H7z"
              clipRule="evenodd"
            />
          </svg>
          <p className={displayDescription ? "inline-block" : "hidden"}>Profit-Loss</p>
        </li>
        <li className="flex gap-x-4 p-2 my-2 border-t-2 dark:hover:bg-dark hover:bg-light hover:bg-opacity-60 rounded">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-5 w-5"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fillRule="evenodd"
              d="M3 3a1 1 0 000 2v8a2 2 0 002 2h2.586l-1.293 1.293a1 1 0 101.414 1.414L10 15.414l2.293 2.293a1 1 0 001.414-1.414L12.414 15H15a2 2 0 002-2V5a1 1 0 100-2H3zm11.707 4.707a1 1 0 00-1.414-1.414L10 9.586 8.707 8.293a1 1 0 00-1.414 0l-2 2a1 1 0 101.414 1.414L8 10.414l1.293 1.293a1 1 0 001.414 0l4-4z"
              clipRule="evenodd"
            />
          </svg>
          <p className={displayDescription ? "inline-block" : "hidden"}>Time Series</p>
        </li>
      </ol>
    </nav>
  );
}
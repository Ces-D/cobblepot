import Head from "next/head";
import React, { useRef, useEffect } from "react";
import Plotly from 'plotly.js-dist-min'

export default function Factory() {
  const elementRef = useRef<HTMLDivElement>(null);

  const createPlot = () => {
    if (elementRef.current) {
      return Plotly.newPlot(
        elementRef.current.id,
        [
          {
            x: [1, 2, 3, 4, 5],
            y: [1, 2, 4, 8, 16],
          },
        ],
        {
          margin: { t: 0 },
        }
      );
    }
  };

  useEffect(() => {
    createPlot();
  }, []);

  return (
    <>
      <div id="pie" ref={elementRef}></div>
    </>
  );
}

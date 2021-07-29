import Head from "next/head";
import React, { useRef, useEffect, createRef } from "react";
import Plotly, { PlotlyHTMLElement, newPlot, Plots } from "plotly.js";

export default function Factory() {
  let elementRef = useRef<HTMLDivElement & PlotlyHTMLElement>();

  useEffect(() => {
    console.log();
    // Plotly.Plots(
    //   elementRef.current.id,
    //   [
    //     {
    //       x: [1, 2, 3, 4, 5],
    //       y: [1, 2, 4, 8, 16],
    //     },
    //   ],
    //   {
    //     margin: { t: 0 },
    //   }
    // );
  }, []);

  return (
    <>
      <div id="graph" ref={elementRef}>
        Hello
      </div>
    </>
  );
}

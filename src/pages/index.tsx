import Head from "next/head";
import Menu from "../components/Menu/Container";

export default function Home() {
  return (
    <>
      <div className="container">
        <Menu />
        Hello
      </div>
    </>
  );
}

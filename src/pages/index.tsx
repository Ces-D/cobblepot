import Head from "next/head";
import Container from "../components/assets/Container";
import Header from "../components/Menu/Header";
import Pie from "../components/Plots/Pie";

export default function Home() {
  return (
    <Container>
      <Header />
      <Pie />
    </Container>
  );
}

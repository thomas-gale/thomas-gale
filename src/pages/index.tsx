import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import config from "../env/config.json";

const Home: NextPage = () => {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center py-2">
      <Head>
        <title>{config.NAME}</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main className="flex w-full flex-1 flex-col items-center justify-center px-20 text-center">
        <h1 className="text-6xl font-bold">
          Personal website for {config.NAME}
        </h1>
        <p className="mt-3 text-2xl">Overview</p>
        <p className="mt-3 text-2xl">Motivations</p>
        <p className="mt-3 text-2xl">Skills</p>
      </main>

      <footer className="flex h-24 w-full items-center justify-center border-t">
        <p className="mt-3 text-2xl">Footer links e.g. github</p>
      </footer>
    </div>
  );
};

export default Home;

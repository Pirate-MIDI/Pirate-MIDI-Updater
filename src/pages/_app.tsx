import type { AppProps } from "next/app";

import "../style.css";

// This default export is required in a new `pages/_app.js` file.
export default function Ahoy({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

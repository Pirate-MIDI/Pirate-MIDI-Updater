const nextConfig = {
  output: "export",
  distDir: "dist",
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true,
  },
};

/* global module */
module.exports = nextConfig;


interface AppConfig {
  name: string;
  version: string;
  author: string;
  description: string;
  main: string;
  scripts: Record<string, string>;
  dependencies: Record<string, string>;
  devDependencies: Record<string, string>;
  peerDependencies: Record<string, string>;
  optionalDependencies: Record<string, string>;
  engines: Record<string, string>;
  os: string[];
  cpu: string[];
  private: boolean;
  workspaces: string[];
  publishConfig: Record<string, string>;
}

const config: AppConfig = {
  name: "midas",
  version: "0.1.0",
  author: "Hadronomy",
  description: "A modern TypeScript project",
  main: "dist/index.js",
  scripts: {
    "build": "tsc",
    "start": "node dist/index.js",
    "dev": "ts-node src/index.ts",
    "test": "jest"
  },
  dependencies: {
    "express": "^4.18.2",
    "typescript": "^5.0.0"
  },
  devDependencies: {
    "@types/node": "^18.0.0",
    "jest": "^29.0.0"
  },
  peerDependencies: {},
  optionalDependencies: {},
  engines: {
    "node": ">=18.0.0"
  },
  os: ["linux", "darwin", "win32"],
  cpu: ["x64", "arm64"],
  private: false,
  workspaces: ["packages/*"],
  publishConfig: {
    "access": "public",
    "registry": "https://registry.npmjs.org/"
  }
};

export default config;

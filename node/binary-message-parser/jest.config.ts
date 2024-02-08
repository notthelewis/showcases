import type { Config } from "@jest/types";

const config: Config.InitialOptions = {
    // Automatically clear mock calls, instances, contexts and results before every test
    clearMocks: true,

    // An array of regexp pattern strings used to skip coverage collection
    coveragePathIgnorePatterns: ["/node_modules/", "dist/"],

    // Indicates which provider should be used to instrument code for coverage
    coverageProvider: "v8",

    // A preset that is used as a base for Jest's configuration
    preset: "ts-jest",

    // The test environment that will be used for testing
    testEnvironment: "node",

    testMatch: ["**/tests/**/*.ts", "**/tests/**/**/*.ts"],
};

export default config;
